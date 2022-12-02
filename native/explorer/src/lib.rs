// MiMalloc won´t compile on Windows with the GCC compiler.
// On Linux with Musl it won´t load correctly.
#[cfg(not(any(
    all(windows, target_env = "gnu"),
    all(target_os = "linux", target_env = "musl")
)))]
use mimalloc::MiMalloc;
use rustler::{Env, Term};

#[cfg(not(any(
    all(windows, target_env = "gnu"),
    all(target_os = "linux", target_env = "musl")
)))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod dataframe;
mod datatypes;
mod encoding;
mod error;
mod expressions;
mod lazyframe;
mod series;

use dataframe::io::*;
use dataframe::*;
pub use datatypes::{
    ExDataFrame, ExDataFrameRef, ExExpr, ExExprRef, ExLazyFrame, ExLazyFrameRef, ExSeries,
    ExSeriesRef,
};
pub use error::ExplorerError;
use expressions::*;
use lazyframe::*;
use series::*;

fn on_load(env: Env, _info: Term) -> bool {
    rustler::resource!(ExDataFrameRef, env);
    rustler::resource!(ExExprRef, env);
    rustler::resource!(ExLazyFrameRef, env);
    rustler::resource!(ExSeriesRef, env);
    true
}

mod atoms {
    rustler::atoms! {
        calendar_iso_module = "Elixir.Calendar.ISO",
        date_module = "Elixir.Date",
        naive_datetime_module = "Elixir.NaiveDateTime",
        hour,
        minute,
        second,
        day,
        month,
        year,
        microsecond,
        calendar,
        nan,
        infinity,
        neg_infinity
    }
}

rustler::init!(
    "Elixir.Explorer.PolarsBackend.Native",
    [
        df_arrange,
        df_arrange_with,
        df_concat_columns,
        df_concat_rows,
        df_describe,
        df_distinct,
        df_drop,
        df_drop_nulls,
        df_dtypes,
        df_dump_csv,
        df_dump_ndjson,
        df_dump_parquet,
        df_dump_ipc,
        df_dump_ipc_stream,
        df_filter_with,
        df_from_csv,
        df_from_ipc,
        df_from_ipc_stream,
        df_from_ndjson,
        df_from_parquet,
        df_from_series,
        df_group_indices,
        df_groups,
        df_head,
        df_join,
        df_load_csv,
        df_load_ndjson,
        df_load_parquet,
        df_load_ipc,
        df_load_ipc_stream,
        df_mask,
        df_mutate_with_exprs,
        df_n_rows,
        df_names,
        df_pivot_longer,
        df_pivot_wider,
        df_pull,
        df_put_column,
        df_rename_columns,
        df_sample_frac,
        df_sample_n,
        df_select,
        df_select_at_idx,
        df_shape,
        df_slice,
        df_slice_by_indices,
        df_slice_by_series,
        df_summarise_with_exprs,
        df_tail,
        df_to_csv,
        df_to_dummies,
        df_to_ipc,
        df_to_ipc_stream,
        df_to_lazy,
        df_to_ndjson,
        df_to_parquet,
        df_width,
        // expressions
        expr_boolean,
        expr_cast,
        expr_column,
        expr_date,
        expr_datetime,
        expr_fill_missing,
        expr_fill_missing_with_value,
        expr_float,
        expr_head,
        expr_integer,
        expr_peaks,
        expr_not,
        expr_sample_frac,
        expr_sample_n,
        expr_series,
        expr_shift,
        expr_slice,
        expr_string,
        expr_tail,
        // sort
        expr_argsort,
        expr_distinct,
        expr_reverse,
        expr_sort,
        expr_unordered_distinct,
        // comparison expressions
        expr_all_equal,
        expr_binary_and,
        expr_binary_or,
        expr_equal,
        expr_greater,
        expr_greater_equal,
        expr_is_nil,
        expr_is_not_nil,
        expr_less,
        expr_less_equal,
        expr_not_equal,
        // arithmetic expressions
        expr_add,
        expr_divide,
        expr_multiply,
        expr_pow,
        expr_quotient,
        expr_remainder,
        expr_subtract,
        // slice and dice expressions
        expr_coalesce,
        expr_concat,
        expr_select,
        // agg expressions
        expr_alias,
        expr_count,
        expr_first,
        expr_last,
        expr_max,
        expr_mean,
        expr_median,
        expr_min,
        expr_n_distinct,
        expr_quantile,
        expr_standard_deviation,
        expr_sum,
        expr_variance,
        // window expressions
        expr_cumulative_max,
        expr_cumulative_min,
        expr_cumulative_sum,
        expr_window_max,
        expr_window_mean,
        expr_window_min,
        expr_window_sum,
        // inspect expressions
        expr_describe_filter_plan,
        // lazyframe
        lf_collect,
        lf_describe_plan,
        lf_drop,
        lf_dtypes,
        lf_fetch,
        lf_head,
        lf_names,
        lf_select,
        lf_tail,
        // series
        s_add,
        s_and,
        s_argsort,
        s_as_str,
        s_cast,
        s_coalesce,
        s_concat,
        s_cumulative_max,
        s_cumulative_min,
        s_cumulative_sum,
        s_distinct,
        s_divide,
        s_dtype,
        s_equal,
        s_fill_missing,
        s_fill_missing_with_bin,
        s_fill_missing_with_boolean,
        s_fill_missing_with_float,
        s_fill_missing_with_int,
        s_greater,
        s_greater_equal,
        s_head,
        s_is_not_null,
        s_is_null,
        s_less,
        s_less_equal,
        s_mask,
        s_max,
        s_mean,
        s_median,
        s_min,
        s_multiply,
        s_n_distinct,
        s_name,
        s_not,
        s_from_list_bool,
        s_from_list_date,
        s_from_list_datetime,
        s_from_list_f64,
        s_from_list_i64,
        s_from_list_u32,
        s_from_list_str,
        s_from_list_binary,
        s_from_binary_f64,
        s_from_binary_i64,
        s_from_binary_i32,
        s_from_binary_u8,
        s_not_equal,
        s_or,
        s_peak_max,
        s_peak_min,
        s_select,
        s_pow_f_lhs,
        s_pow_f_rhs,
        s_pow_i_lhs,
        s_pow_i_rhs,
        s_quantile,
        s_quotient,
        s_remainder,
        s_rename,
        s_reverse,
        s_seedable_random_indices,
        s_series_equal,
        s_shift,
        s_size,
        s_slice,
        s_slice_by_indices,
        s_sort,
        s_standard_deviation,
        s_subtract,
        s_sum,
        s_tail,
        s_at,
        s_at_every,
        s_to_list,
        s_to_iovec,
        s_unordered_distinct,
        s_frequencies,
        s_variance,
        s_window_max,
        s_window_mean,
        s_window_min,
        s_window_sum,
    ],
    load = on_load
);
