use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use rocket::serde::json::Json;
use rust_wheel::common::query::pagination::PaginateForQueryFragment;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use crate::model::request::app::cruise::article::article_request::ArticleRequest;
use crate::models::Favorites;
use rust_wheel::config::db::config;

pub fn article_query<T>(request: &Json<ArticleRequest>) -> PaginationResponse<Vec<Favorites>> {
    use crate::model::diesel::rhythm::rhythm_schema::favorites::dsl::*;
    let connection = config::establish_music_connection();
    let query = favorites.filter(like_status.eq(1))
        .paginate(request.pageNum)
        .per_page(request.pageSize);
    let query_result: QueryResult<(Vec<_>, i64, i64)> = query.load_and_count_pages_total::<Favorites>(&connection);
    let page_result = map_pagination_res(query_result, request.pageNum, request.pageSize);
    return page_result;
}