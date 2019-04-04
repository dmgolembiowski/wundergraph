use crate::helper::*;
use wundergraph_example::MyContext;

#[test]
fn query_filter_eq_not_nullable_child() {
    let (schema, pool) = get_example_schema();
    let ctx = MyContext::new(pool.get().unwrap());

    let res = execute_query(
        &schema,
        &ctx,
        r#"
{
    Heros(filter: {species: {name: {eq: "Human"}}}) {
        heroName
    }
}
"#,
    );

    assert!(res.is_ok());
    assert_eq!(
        json!([{"Heros": [
            {"heroName": "Luke Skywalker"},
            {"heroName": "Darth Vader"},
            {"heroName": "Han Solo"},
            {"heroName": "Leia Organa"},
            {"heroName": "Wilhuff Tarkin"}
    ]}, []]),
        res.as_json()
    );
}

#[test]
fn query_filter_eq_nullable_child() {
    let (schema, pool) = get_example_schema();
    let ctx = MyContext::new(pool.get().unwrap());

    let res = execute_query(
        &schema,
        &ctx,
        r#"
{
    Heros(filter: {home_world: {name: {eq: "Alderaan"}}}) {
        heroName
    }
}
"#,
    );

    assert!(res.is_ok());
    assert_eq!(
        json!([{"Heros": [
            {"heroName": "Leia Organa"},

        ]}, []]),
        res.as_json()
    );
}

#[test]
fn query_filter_nullable_child_is_null() {
    let (schema, pool) = get_example_schema();
    let ctx = MyContext::new(pool.get().unwrap());

    let res = execute_query(
        &schema,
        &ctx,
        r#"
{
    Heros(filter: {home_world: {is_null: true}}) {
        heroName
    }
}
"#,
    );

    assert!(res.is_ok());
    assert_eq!(
        json!([{"Heros": [
            {"heroName": "Han Solo"},
            {"heroName": "Wilhuff Tarkin"}
        ]}, []]),
        res.as_json()
    );
}

#[test]
fn query_filter_negative_expression() {
    let (schema, pool) = get_example_schema();
    let ctx = MyContext::new(pool.get().unwrap());

    let res = execute_query(
        &schema,
        &ctx,
        r#"
{
    Heros(filter: {home_world: {name: {not_eq: "Tatooine"}}}) {
        heroName
    }
}
"#,
    );

    assert!(res.is_ok());
    // Only Leia has a home_world that is set and not "Tatooine"
    assert_eq!(
        json!([{"Heros": [
            {"heroName": "Leia Organa"},
        ]}, []]),
        res.as_json()
    );
}

#[test]
fn query_filter_double_nested() {
    let (schema, pool) = get_example_schema();
    let ctx = MyContext::new(pool.get().unwrap());

    let res = execute_query(
        &schema,
        &ctx,
        r#"
{
    Heros(filter: {home_world: {heros: {heroName: {like: "Luke%"}}}}) {
        heroName
    }
}
"#,
    );
    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(
        json!([{"Heros": [
            {"heroName": "Luke Skywalker"},
            {"heroName": "Darth Vader"}
        ]}, []]),
        res.as_json()
    );
}

#[test]
fn query_filter_double_nested_negative() {
    let (schema, pool) = get_example_schema();
    let ctx = MyContext::new(pool.get().unwrap());

    let res = execute_query(
        &schema,
        &ctx,
        r#"
{
    Heros(filter: {home_world: {heros: {heroName: {not_eq: "Leia Organa"}}}}) {
        heroName
    }
}
"#,
    );
    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(
        json!([{"Heros": [
            {"heroName": "Luke Skywalker"},
            {"heroName": "Darth Vader"}
        ]}, []]),
        res.as_json()
    );
}