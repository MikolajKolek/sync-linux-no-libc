#![no_std]

#[cfg(test)]
mod tests {
    use nc::{getpid, SIGUSR1};

    #[test]
    fn example_test() {
        unsafe {
            nc::kill(getpid(), SIGUSR1).unwrap()
        }
    }
}
