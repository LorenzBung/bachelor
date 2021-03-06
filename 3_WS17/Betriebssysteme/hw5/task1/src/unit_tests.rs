#[cfg(test)]
mod tests {

    use child::*;
    use procinfo;

    #[test]
    fn test_zero_forks() {
        assert_eq!(run_childs(123, "0"), Ok(()))
    }

    #[test]
    fn test_one_fork() {
        assert_eq!(
            run_childs(procinfo::pid::stat_self().unwrap().pid, "1"),
            Ok(())
        )
    }

    #[test]
    fn test_wrong_pid() {
        assert_eq!(run_childs(2, "1"), Ok(()))
    }
}
