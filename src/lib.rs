#![allow(non_camel_case_types)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_fn_trait_return)]
#![feature(impl_trait_in_assoc_type)]
#![feature(const_for)]
#![feature(const_trait_impl)]

use core::{Applicative, Functor, Monad, Unwrap};
use std::{future::{self, Future}, ops::Deref, pin::Pin, time::Duration};
use anyhow::{anyhow, Result};
use futures::{FutureExt};
use tokio::time::sleep;

pub mod experimental;

pub mod core;

#[macro_use]
pub mod macros;

#[macro_use]
pub mod type_macros;


pub trait MonadError<'a, E> : Monad<'a>
{
    fn throwError<A>(e : E) -> Self::of<A>;
    fn catchError<A>(action: Self::of<A>, handler: impl FnOnce(E) -> Self::of<A> + 'static) -> Self::of<A>;
}

pub struct Task<A>(Pin<Box<dyn Future<Output = Result<A>>>>);

pub struct TaskF();
impl<A: 'static> Unwrap<'static> for Task<A>
{
    type A = A;
    type F = TaskF;
    
    fn coe(self) -> <Self::F as Functor<'static>>::of<Self::A> {
        self
    }
    
    fn uncoe(x: <Self::F as Functor<'static>>::of<Self::A>) -> Self {
        x
    }
}

// fn map<'a, A: 'a ,B: 'a ,F>(f: F, a: Task<A>) -> Task<B>
//     where F: Fn(A) -> B 
// {
//     let x = a.0.map(|x| x.map(f));
//     // let x = a.0.map(f);
//     Task(Box::pin(x))
//     //     Box::pin(async {
//     //     match (*a.0).await {
//     //     }
//     //     Ok(f(a.0))
//     // })
//     // Box::new(a.0.deref().)
// }


impl Functor<'static> for TaskF
{
    type of<A: 'static> = Task<A>;

    fn map<A,B,F>(f: F, a: Self::of<A>) -> Self::of<B>
        where F: Fn(A) -> B + 'static,
            A: 'static,
            B: 'static
    {
        Task(Box::pin(a.0.map(|x| x.map(f))))
    }
}

impl Applicative<'static> for TaskF
{

    fn pure<A: 'static>(a: A) -> Self::of<A> {
        Task(Box::pin(future::ready(Ok(a))))
    }
    
    fn funmap<A: 'static, B: 'static>(f: Self::of<impl Fn(A) -> B + 'static + 'static + Copy>) -> impl Fn(Self::of<A>) -> Self::of<B> {
        |a| todo!()
    }
}

impl Monad<'static> for TaskF
{

    fn bind<A,B,F>(a: Self::of<A>, f: F) -> Self::of<B>
        where
            F : Fn(A) -> Self::of<B>,
            A: 'static,
            B: 'static,
            F: 'static
    {
        Task(Box::pin(async move {
            match a.0.await {
                Ok(x) => f(x).0.await,
                Err(err) => Err(err),
            }
        }))
    }
}

// impl<A> Functor for 
// pub struct TaskResult<A>(Result)


pub trait TaskMonad : MonadError<'static, anyhow::Error>
{
    fn try_task<Fut: Future<Output=Result<A>> + 'static, A>(fut: Fut) -> Self::of<A>;

    fn task<Fut: Future + 'static>(fut: Fut) -> Self::of<Fut::Output>
    {
        Self::try_task(async {Ok(fut.await)})
    }
}

impl MonadError<'static, anyhow::Error> for TaskF
{
    fn throwError<A: 'static>(e : anyhow::Error) -> Self::of<A> {
        Task(Box::pin(future::ready(Err(e))))
    }
    
    fn catchError<A: 'static>(action: Self::of<A>, handler: impl FnOnce(anyhow::Error) -> Self::of<A> + 'static) -> Self::of<A> {
        Task(Box::pin(async move {
            match action.0.await {
                Ok(x) => Ok(x),
                Err(err) => handler(err).0.await,
            }
        }))
    }
}

impl TaskMonad for TaskF
{
    fn try_task<Fut: Future<Output=Result<A>> + 'static, A: 'static>(fut: Fut) -> Self::of<A> {
        Task(Box::pin(fut))
    }
}

pub trait Get<A> : Fn() -> A + 'static + Clone {}
impl<X: Fn() -> A + 'static + Clone, A> Get<A> for X {}

#[derive(Clone, Copy)]
struct with 
{
    times: usize
}

fn retry<M: TaskMonad, A: 'static>
(
    a: impl Get<M::of<A>>,
    with: with
)
-> M::of<A>
{
    M::catchError(a(), move |_| {
        if with.times <= 0
        {
            M::throwError(anyhow!("Run out of times to retry"))
        }
        else
        {
            M::bind(
            M::task( sleep(Duration::from_secs(1))),
            move |_| retry::<M,_>(a.clone(), with { times: with.times - 1})
            )
        }
    })
}

fn mytest<M: TaskMonad>() -> M::of<i32>
{
    retry::<M,_>(|| M::throwError(anyhow!("")), with { times: 4})
    // M::retry(times: 4) $ M::try_task(err)
}



