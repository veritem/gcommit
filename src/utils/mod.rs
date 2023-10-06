pub fn build_commit_message(
    scope: &Option<String>,
    change: &Option<String>,
    message: &Option<String>,
) -> Option<String> {
    let mut commit_message: String = "".to_owned();
    {
        match change {
            Some(s) => commit_message.push_str(s),
            None => {
                panic!("Commit classification  is required to maintain git commit conventions")
            }
        }

        match scope {
            Some(s) => commit_message.push_str(format!("({})", s).as_str()),
            None => {}
        }

        match message {
            Some(s) => {
                commit_message.push_str(":");
                commit_message.push_str(s)
            }
            None => {
                panic!("You didn't add a commit message")
            }
        }
    }

    println!("{}", commit_message.trim().len());
    let result = if commit_message.trim().len() == 0 {
        None
    } else {
        Some(String::from(commit_message))
    };

    result
}
