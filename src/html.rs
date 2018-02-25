pub mod html {
    pub const form: &str = r#"
    <!doctype html>
    <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">
    <title>Conversione Decimale-Binaria Binaria-Decimale</title>
    <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0-beta.2/css/bootstrap.min.css" integrity="sha384-PsH8R72JQ3SOdhVi3uxftmaW6Vc51MKb0q5P2rRUpPvrszuE4W1povHYgTpBfshb" crossorigin="anonymous">
    </head>
    <body>
  <main role="main" class="container">
      <h1>Conversione Decimale-Binaria Binaria-Decimale</h1>
      <p class="lead">Inserisci un numero da convertire e che conversione da fare.</p>
    <div>
      <label>Cerca scuole per</label>
      <form method="get" action="/search">
          <label class="radio-inline">Decimale-Binaria<input type="radio" class="form-control" name="scelta" value="decbin" checked></label>
          <label class="radio-inline">Binaria-Decimale<input type="radio" class="form-control" name="scelta" value="bindec"></label>
        <div class="form-group">
          <label for="text">Numero da convertire </label>
          <input type="text" name="n"><br>
        </div>
     <button type="submit" class="btn btn-default">Cerca</button>
    </form>
  </div>
  </main><!-- /.container -->
  <!-- Bootstrap core JavaScript-->
  <script src="https://code.jquery.com/jquery-3.2.1.slim.min.js" integrity="sha384-KJ3o2DKtIkvYIK3UENzmM7KCkRr/rE9/Qpg6aAZGJwFDMVNA/GpGFF93hXpG5KkN" crossorigin="anonymous"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.12.3/umd/popper.min.js" integrity="sha384-vFJXuSJphROIrBnz7yo7oB41mKfc8JzQZiCq4NCceLEaO4IHwicKwpJf9c9IpFgh" crossorigin="anonymous"></script>
  <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0-beta.2/css/bootstrap.min.css" integrity="sha384-PsH8R72JQ3SOdhVi3uxftmaW6Vc51MKb0q5P2rRUpPvrszuE4W1povHYgTpBfshb" crossorigin="anonymous">
  <script src="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0-beta.2/js/bootstrap.min.js" integrity="sha384-alpBpkh1PFOepccYVYDB4do5UnbKysX5WZXm3XxPqe5iKTfUKjNkCk9SaVuEZflJ" crossorigin="anonymous"></script>
    </body>
    "#;
}
