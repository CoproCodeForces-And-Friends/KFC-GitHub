extern crate serde_json;
extern crate hyper;
extern crate failure;

#[allow(non_snake_case)]
#[derive(Serialize)]
struct KfcTask {
    id: i32,
    name: String,
    description: String,
    creationDate: String,
    status: String,
    creatorId: i32,
    handlerId: i32,
    due: String,
    label: Vec<String>,
    projectId: String,
}

// Not sure if required
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Serialize)]
struct KfcRelatedTask {
    id: i32,
    typee: String,
}

// Not sure if required
#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Serialize)]
struct KfcActivity {
    memberId: i32,
    typee: String,
    date: String,
    oldValue: String,
    newValue: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct KfcOrganization {
    id: i32,
    name: String,
    description: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct KfcParticipant {
    id: i32,
    name: String,
    email: String,
}

#[cfg(test)]
mod test {
    use failure::Error;
    use std::fs::File;
    use serde_json as json;

    #[derive(Fail, Debug)]
    enum FileJsonError {
        #[fail(display = "Json error: {:?}", _0)]
        Json(#[cause] json::Error),

        #[fail(display = "IO error: {:?}", _0)]
        Io(#[cause] ::std::io::Error),
    }

    impl From<::std::io::Error> for FileJsonError {
        fn from(e: ::std::io::Error) -> Self {
            FileJsonError::Io(e)
        }
    }

    impl From<json::Error> for FileJsonError {
        fn from(e: json::Error) -> Self {
            FileJsonError::Json(e)
        }
    }

    fn json_from_file(p: &'static str) -> Result<json::Value, Error> {
        let file = File::open(p)?;
        let value = json::from_reader(file)?;
        Ok(value)
    }

    #[test]
    fn test_gh_issue_extacted() {
        let val = json_from_file("resources/label_add.json").unwrap();
        assert!(val.pointer("/issue/id").map(|x| x.is_number()).unwrap_or_default());
    }
}
