// == File for API answers ==

use axum::http::StatusCode;



pub enum ApiAnswersEnum {
    Answer1,
    Answer2,
    Answer3,
    Answer4,
    Answer5,
    Answer6,
    Answer7,
}



pub struct ApiAnswers {
    _name: ApiAnswersEnum,
    pub code: StatusCode,
    pub message: String,
}



impl ApiAnswers {
    pub fn new(answer_name: ApiAnswersEnum) -> ApiAnswers {
        let answer_type = match answer_name {
            ApiAnswersEnum::Answer1 => String::from("データの登録を完了した"),
            ApiAnswersEnum::Answer2 => String::from("データが既にある"),
            ApiAnswersEnum::Answer3 => String::from("データが登録不可能になっている"),
            ApiAnswersEnum::Answer4 => String::from("データが存在している"),
            ApiAnswersEnum::Answer5 => String::from("データが見つからない"),
            ApiAnswersEnum::Answer6 => String::from("データが登録不可能になっている"),
            ApiAnswersEnum::Answer7 => String::from("データが登録可能になっている"),
            
        };

        let answer_code = match answer_name {
            ApiAnswersEnum::Answer1 => StatusCode::OK,
            ApiAnswersEnum::Answer2 => StatusCode::NOT_ACCEPTABLE,
            ApiAnswersEnum::Answer3 => StatusCode::NOT_IMPLEMENTED,
            ApiAnswersEnum::Answer4 => StatusCode::OK,
            ApiAnswersEnum::Answer5 => StatusCode::NOT_FOUND,
            ApiAnswersEnum::Answer6 => StatusCode::NOT_FOUND,
            ApiAnswersEnum::Answer7 => StatusCode::FOUND,
        };

        ApiAnswers {
            _name: answer_name,
            code: answer_code,
            message:answer_type
        }
    }
}