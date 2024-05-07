use std::time::Duration;
use leptos_query::{QueryOptions, ResourceOption};

//Default configuration options for a query
pub fn get_query_options<T>() ->QueryOptions<Result<Vec<T>, String>>{
    let q: QueryOptions<Result<Vec<T>, String>> = QueryOptions{
        default_value: None,
        stale_time: None,
        gc_time: None,
        refetch_interval: Some(Duration::from_secs(60)),
        resource_option: Some(ResourceOption::Local),
    };
    q
}