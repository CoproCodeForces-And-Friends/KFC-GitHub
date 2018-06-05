extern crate failure;
extern crate hyper;
extern crate serde_json;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct KfcTask {
    id: i32,
    name: String,
    description: String,
    creation_date: String,
    status: String,
    creator_id: i32,
    handler_id: i32,
    due: String,
    label: Vec<String>,
    project_id: String,
}

// Not sure if required
#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct KfcRelatedTask {
    id: i32,
    #[serde(rename = "type")]
    type_: String,
}

// Not sure if required
#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct KfcActivity {
    member_id: i32,
    #[serde(rename = "type")]
    type_: String,
    date: String,
    old_value: String,
    new_value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct KfcOrganization {
    id: i32,
    name: String,
    description: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct KfcParticipant {
    id: i32,
    name: String,
    email: String,
}

#[cfg(test)]
mod test {
    use failure::Error;
    use serde_json as json;
    use std::fs::File;

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
        assert!(
            val.pointer("/issue/id")
                .map(|x| x.is_number())
                .unwrap_or_default()
        );
    }
}
