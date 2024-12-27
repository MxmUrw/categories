use crate::define_many;
use crate::def;


trait Fibered
{
    type Base;

}

trait StateMachined
{
    type Input;
    type Output;
}

def!
{
    [ trait Monoid where
    |
    | A : type
    | B : type
    |
    | hello : A -> B = () 
    | 
    ]

    [ trait Fibered where
    | Base : type
    | is_base : self (base: Base) -> bool
    ]

    [ trait StateMachine where

    /// input
    | I : Set, Fibered

    /// output
    | O : Set

    /// current state
    | S : Set

    /// transition function
    | δ (state: &mut S) (input : I) : O
    where
      | if state.valid() && input.valid()
      | => |result| result.valid()   

    
    //
    //
    ]
}