use regex::Regex;
use uuid::Uuid;

use crate::{
    AppContext,
    error::{AppError, AppResult},
    models::collection::{Collection, CollectionSummary, CreateCollectionInput, UpdateCollectionInput},
};

pub async fn list_collections(context: &AppContext) -> AppResult<Vec<CollectionSummary>> {
    let rows = sqlx::query_as::<_, CollectionSummary>(
        r#"
        select
          c.id,
          c.name,
          c.slug,
          c.accent_color,
          c.last_digest_at,
          count(distinct sc.subscription_id) as source_count,
          count(distinct e.id) filter (where coalesce(es.is_read, false) = false) as unread_count
        from collections c
        left join subscription_collections sc on sc.collection_id = c.id
        left join subscriptions s on s.id = sc.subscription_id
        left join entries e on e.source_id = s.source_id
        left join entry_states es on es.entry_id = e.id
        group by c.id
        order by c.name asc
        "#,
    )
    .fetch_all(&context.pool)
    .await?;
    Ok(rows)
}

pub async fn create_collection(
    context: &AppContext,
    input: &CreateCollectionInput,
) -> AppResult<Collection> {
    if input.name.trim().is_empty() {
        return Err(AppError::validation("collection name cannot be empty"));
    }
    let slug = slugify(&input.name);
    let collection = sqlx::query_as::<_, Collection>(
        r#"
        insert into collections (id, name, slug, accent_color)
        values ($1, $2, $3, $4)
        returning
          id, name, slug, accent_color, last_digest, last_digest_at, created_at, updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(input.name.trim())
    .bind(slug)
    .bind(input.accent_color.trim())
    .fetch_one(&context.pool)
    .await?;
    Ok(collection)
}

pub async fn update_collection(
    context: &AppContext,
    collection_id: Uuid,
    input: &UpdateCollectionInput,
) -> AppResult<Collection> {
    let current = fetch_collection(context, collection_id).await?;
    let name = input.name.clone().unwrap_or(current.name);
    let accent_color = input
        .accent_color
        .clone()
        .unwrap_or(current.accent_color);
    let slug = slugify(&name);
    let collection = sqlx::query_as::<_, Collection>(
        r#"
        update collections
        set name = $2, slug = $3, accent_color = $4, updated_at = now()
        where id = $1
        returning
          id, name, slug, accent_color, last_digest, last_digest_at, created_at, updated_at
        "#,
    )
    .bind(collection_id)
    .bind(name)
    .bind(slug)
    .bind(accent_color)
    .fetch_one(&context.pool)
    .await?;
    Ok(collection)
}

pub async fn delete_collection(context: &AppContext, collection_id: Uuid) -> AppResult<()> {
    let result = sqlx::query("delete from collections where id = $1")
        .bind(collection_id)
        .execute(&context.pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::not_found("collection not found"));
    }
    Ok(())
}

async fn fetch_collection(context: &AppContext, collection_id: Uuid) -> AppResult<Collection> {
    sqlx::query_as::<_, Collection>(
        r#"
        select
          id, name, slug, accent_color, last_digest, last_digest_at, created_at, updated_at
        from collections
        where id = $1
        "#,
    )
    .bind(collection_id)
    .fetch_optional(&context.pool)
    .await?
    .ok_or_else(|| AppError::not_found("collection not found"))
}

fn slugify(input: &str) -> String {
    let regex = Regex::new(r"[^a-z0-9]+").expect("valid regex");
    let lower = input.trim().to_lowercase();
    let slug = regex.replace_all(&lower, "-");
    slug.trim_matches('-').to_owned()
}

#[cfg(test)]
mod tests {
    use super::slugify;

    #[test]
    fn slugify_strips_symbols() {
        assert_eq!(slugify("AI & Rust Daily"), "ai-rust-daily");
    }
}
