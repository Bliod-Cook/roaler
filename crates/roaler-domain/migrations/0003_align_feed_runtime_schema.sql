alter table entries
  add column if not exists author_name text,
  add column if not exists content_seed_html text;

do $$
begin
  if exists (
    select 1
    from information_schema.columns
    where table_name = 'entries' and column_name = 'author'
  ) then
    execute $sql$
      update entries
      set author_name = author
      where author_name is null and author is not null
    $sql$;
  end if;
end $$;

alter table entry_contents
  add column if not exists html_content text,
  add column if not exists status content_status not null default 'pending',
  add column if not exists error_message text,
  add column if not exists extracted_at timestamptz;

do $$
begin
  if exists (
    select 1
    from information_schema.columns
    where table_name = 'entry_contents' and column_name = 'content_html'
  ) then
    execute $sql$
      update entry_contents
      set html_content = content_html
      where html_content is null and content_html is not null
    $sql$;
  end if;

  if exists (
    select 1
    from information_schema.columns
    where table_name = 'entry_contents' and column_name = 'content_status'
  ) then
    execute $sql$
      update entry_contents
      set status = content_status
    $sql$;
  end if;

  if exists (
    select 1
    from information_schema.columns
    where table_name = 'entry_contents' and column_name = 'content_error'
  ) then
    execute $sql$
      update entry_contents
      set error_message = content_error
      where error_message is null and content_error is not null
    $sql$;
  end if;
end $$;

alter table sync_runs
  alter column fetched_count set default 0,
  alter column inserted_count set default 0;
