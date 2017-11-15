(function(Elm){
    var container = document.getElementById('container');
    var app = Elm.Main.embed(container);

    app.ports.websocket_address.send('ws://127.0.0.1:3436');
})(Elm);
