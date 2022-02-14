pub trait Handler0: Clone + 'static {
    type Output;
    fn call(&self) -> Self::Output;
}

impl<Func, Return> Handler0 for Func
where
    Func: Fn() -> Return + Clone + 'static,
{
    type Output = Return;

    #[inline]
    fn call(&self) -> Self::Output {
        (self)()
    }
}

pub trait Handler1<Arg>: Clone + 'static {
    type Output;
    fn call(&self, arg: Arg) -> Self::Output;
}

impl<Func, Return, A> Handler1<A> for Func
where
    Func: Fn(A) -> Return + Clone + 'static,
{
    type Output = Return;

    #[inline]
    fn call(&self, arg: A) -> Self::Output {
        (self)(arg)
    }
}

pub trait Handler2<Arg1, Arg2>: Clone + 'static {
    type Output;
    fn call(&self, arg1: Arg1, arg2: Arg2) -> Self::Output;
}

impl<Func, Return, A, B> Handler2<A, B> for Func
where
    Func: Fn(A, B) -> Return + Clone + 'static,
{
    type Output = Return;

    #[inline]
    fn call(&self, arg1: A, arg2: B) -> Self::Output {
        (self)(arg1, arg2)
    }
}

pub trait Handler3<Arg1, Arg2, Arg3>: Clone + 'static {
    type Output;
    fn call(&self, arg1: Arg1, arg2: Arg2, arg3: Arg3) -> Self::Output;
}

impl<Func, Return, A, B, C> Handler3<A, B, C> for Func
where
    Func: Fn(A, B, C) -> Return + Clone + 'static,
{
    type Output = Return;

    #[inline]
    fn call(&self, arg1: A, arg2: B, arg3: C) -> Self::Output {
        (self)(arg1, arg2, arg3)
    }
}

pub trait Handler4<Arg1, Arg2, Arg3, Arg4>: Clone + 'static {
    type Output;
    fn call(&self, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4) -> Self::Output;
}

impl<Func, Return, A, B, C, D> Handler4<A, B, C, D> for Func
where
    Func: Fn(A, B, C, D) -> Return + Clone + 'static,
{
    type Output = Return;

    #[inline]
    fn call(&self, arg1: A, arg2: B, arg3: C, arg4: D) -> Self::Output {
        (self)(arg1, arg2, arg3, arg4)
    }
}

pub trait Handler5<Arg1, Arg2, Arg3, Arg4, Arg5>: Clone + 'static {
    type Output;
    fn call(&self, arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5) -> Self::Output;
}

impl<Func, Return, A, B, C, D, E> Handler5<A, B, C, D, E> for Func
where
    Func: Fn(A, B, C, D, E) -> Return + Clone + 'static,
{
    type Output = Return;

    #[inline]
    fn call(&self, arg1: A, arg2: B, arg3: C, arg4: D, arg5: E) -> Self::Output {
        (self)(arg1, arg2, arg3, arg4, arg5)
    }
}

fn test0() -> i32 {
    42
}

fn test1(a: i32) -> i32 {
    a + 42
}

fn test2(a: i32, b: i32) -> i32 {
    a + b + 42
}

fn test3(a: i32, b: i32, c: i32) -> i32 {
    a + b + c + 42
}

fn test4(a: i32, b: i32, c: i32, d: i32) -> i32 {
    a + b + c + d + 42
}

fn test5(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    a + b + c + d + e + 42
}

fn test0_no_return() {
    ()
}

pub trait Handler<Args>: Clone + 'static {
    type Output;
    fn call(&self, args: Args) -> Self::Output;
}

impl<Func, Return> Handler<()> for Func
where
    Func: Fn() -> Return + Clone + 'static,
{
    type Output = Return;
    #[inline]
    fn call(&self, _arg: ()) -> Self::Output {
        (self)()
    }
}

impl<Func, Return, A> Handler<(A,)> for Func
where
    Func: Fn(A) -> Return + Clone + 'static,
{
    type Output = Return;
    #[inline]
    fn call(&self, arg: (A,)) -> Self::Output {
        (self)(arg.0)
    }
}

impl<Func, Return, A, B> Handler<(A, B)> for Func
where
    Func: Fn(A, B) -> Return + Clone + 'static,
{
    type Output = Return;
    #[inline]
    fn call(&self, arg: (A, B)) -> Self::Output {
        (self)(arg.0, arg.1)
    }
}

// macro_rules! factory_tuple ({ $($param:ident)* } => {
//     impl<Func, Return, $($param,)*> Handler<($($param,)*)> for Func
//     where
//         Func: Fn($($param),*) -> Return + Clone + 'static,
//     {
//         type Output = Return;
//
//         #[inline]
//         #[allow(non_snake_case)]
//         fn call(&self, ($($param,)*): ($($param,)*)) -> Self::Output {
//             (self)($($param,)*)
//         }
//     }
// });

// factory_tuple! {}
// factory_tuple! { A }
// factory_tuple! { A B }

fn take1<F, Args>(f: F)
where
    F: Handler1<Args>,
    Args: 'static,
    F::Output: 'static,
{
}

fn take2<F, Args>(f: F)
    where
        F: Handler<Args>,
        Args: Send + 'static,
        F::Output: 'static,
{
//    f.call(? Args);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        take2(test0);
        take2(test1);
        take2(test2);
    }
}
