//! Utility module to generate a GraphiQL interface

/// Generate the HTML source to show a GraphiQL interface
pub fn graphiql_source(graphql_endpoint_url: &str) -> String {
    let stylesheet_source = r#"
    <style>
        html, body, #app {
            height: 100%;
            margin: 0;
            overflow: hidden;
            width: 100%;
        }
    </style>
    "#;
    let fetcher_source = r#"
    <script>
        function graphQLFetcher(params) {
            return fetch(GRAPHQL_URL, {
                method: 'post',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(params)
            }).then(function (response) {
                return response.text();
            }).then(function (body) {
                try {
                    return JSON.parse(body);
                } catch (error) {
                    return body;
                }
            });
        }
        ReactDOM.render(
            React.createElement(GraphiQL, {
                fetcher: graphQLFetcher,
            }),
            document.querySelector('#app'));
    </script>
    "#;

    format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>GraphQL</title>
    {stylesheet_source}
    <link rel="stylesheet" type="text/css" href="//cdnjs.cloudflare.com/ajax/libs/graphiql/0.10.2/graphiql.css">
</head>
<body>
    <div id="app"></div>

    <script src="//cdnjs.cloudflare.com/ajax/libs/fetch/2.0.3/fetch.js"></script>
    <script src="//cdnjs.cloudflare.com/ajax/libs/react/15.6.0/react.js"></script>
    <script src="//cdnjs.cloudflare.com/ajax/libs/react/15.6.0/react-dom.js"></script>
    <script src="//cdnjs.cloudflare.com/ajax/libs/graphiql/0.10.2/graphiql.js"></script>
    <script>var GRAPHQL_URL = '{graphql_url}';</script>
    {fetcher_source}
</body>
</html>
"#,
        graphql_url = graphql_endpoint_url,
        stylesheet_source = stylesheet_source,
        fetcher_source = fetcher_source)

}