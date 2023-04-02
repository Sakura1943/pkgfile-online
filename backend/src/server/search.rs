use poem::{
    error::ParseQueryError,
    handler,
    http::StatusCode,
    web::{Json, Query},
    IntoResponse, Response, Result,
};
use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::server::utils::RequestFailed;

#[derive(Debug, Clone, Serialize)]
pub struct ResponseMessage {
    pub status: u16,
    pub data: Vec<ResponseData>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseData {
    pub repo: String,
    pub name: String,
    pub version: String,
    pub path: Vec<String>,
}

impl ResponseMessage {
    pub fn new(file_name: &str) -> Result<Self> {
        Ok(Self {
            status: 0,
            data: sp_to_fields(file_name)?,
        })
    }
    pub fn status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }
}

fn read_stdout(file_name: &str) -> anyhow::Result<Vec<String>> {
    let stdout = String::from_utf8_lossy(
        &Command::new("pkgfile")
            .args(["-v", file_name])
            .output()?
            .stdout,
    )
    .to_string();
    Ok(stdout
        .lines()
        .map(|el| el.trim().to_string())
        .collect::<Vec<String>>())
}

fn sp_to_fields(file_name: &str) -> Result<Vec<ResponseData>> {
    let lines = read_stdout(file_name)?;
    let mut datas: Vec<ResponseData> = vec![];
    for line in lines {
        let sps = line
            .split_whitespace()
            .map(|el| el.trim())
            .collect::<Vec<&str>>();
        let repo_name = sps[0].split('/').collect::<Vec<&str>>();
        let repo = repo_name[0].to_owned();
        let name = repo_name[1].to_owned();
        let version = sps[1].to_owned();
        let paths = vec![sps[2].to_owned()];

        let mut equal = false;

        if datas.len().gt(&0) {
            for (index, data) in datas.clone().iter().enumerate() {
                if format!("{}/{}", &data.repo, data.name).eq(&format!("{repo}/{name}")) {
                    datas[index].path.extend(paths.clone());
                    equal = true;
                }
            }
        }

        if !equal {
            datas.push(ResponseData {
                repo,
                name,
                version,
                path: paths,
            });
        }
    }

    Ok(datas)
}

#[derive(Debug, Deserialize)]
pub struct Params {
    f: String,
}

#[handler]
pub fn search(res: Result<Query<Params>>) -> Result<impl IntoResponse> {
    match res {
        Ok(Query(params)) => {
            let file_name = params.f;
            let message = ResponseMessage::new(&file_name)?.status(StatusCode::OK.as_u16());
            if message.data.len().eq(&0) {
                Ok(Json(
                    RequestFailed::default()
                        .with_error_msg("Packages not found")
                        .with_status(StatusCode::BAD_REQUEST),
                )
                .with_status(StatusCode::BAD_REQUEST)
                .into_response())
            } else {
                Ok(Json(message).with_status(StatusCode::OK).into_response())
            }
        }
        Err(err) if err.is::<ParseQueryError>() => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(err.to_string())),
        Err(err) => Err(err),
    }
}
