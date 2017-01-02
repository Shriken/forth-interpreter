use forth;
use forth::error::Error;

type NativeWordFunc = fn (&mut forth::ValueStack) -> Result<(), Error>;

pub const NATIVE_WORDS: &'static [(&'static str, NativeWordFunc)] = &[
    (".", pop_and_print),
    ("+", add),
    ("-", sub),
    ("*", mul),
    ("/", div),
];

macro_rules! try_pop_n {
    ($stack:expr, $n:expr) => {{
        let l = $stack.len();
        if l < $n {
            $stack.clear();
            return Err(Error::from(
                format!("expected {} arguments, got {}", $n, l)
            ));
        } else {
            $stack.split_off(l - $n)
        }
    }}
}

fn pop_and_print(stack: &mut forth::ValueStack) -> Result<(), Error> {
    match stack.pop() {
        Some(n) => {
            print!("{} ", n);
            Ok(())
        },
        None => Err(Error::from("expected 1 argument")),
    }
}

fn add(stack: &mut forth::ValueStack) -> Result<(), Error> {
    let args = try_pop_n!(stack, 2);
    stack.push(args[0] + args[1]);
    Ok(())
}

fn sub(stack: &mut forth::ValueStack) -> Result<(), Error> {
    let args = try_pop_n!(stack, 2);
    stack.push(args[0] - args[1]);
    Ok(())
}

fn mul(stack: &mut forth::ValueStack) -> Result<(), Error> {
    let args = try_pop_n!(stack, 2);
    stack.push(args[0] * args[1]);
    Ok(())
}

fn div(stack: &mut forth::ValueStack) -> Result<(), Error> {
    let args = try_pop_n!(stack, 2);
    stack.push(args[0] / args[1]);
    Ok(())
}

#[test]
fn test_math() {
    {
        let mut v = vec![1, 1];
        assert!(add(&mut v).is_ok());
        assert_eq!(v, vec![2]);
    }

    {
        let mut v = vec![3, 2];
        assert!(sub(&mut v).is_ok());
        assert_eq!(v, vec![1]);
    }

    {
        let mut v = vec![2, 3];
        assert!(mul(&mut v).is_ok());
        assert_eq!(v, vec![6]);
    }

    {
        let mut v = vec![6, 3];
        assert!(div(&mut v).is_ok());
        assert_eq!(v, vec![2]);
    }
}
