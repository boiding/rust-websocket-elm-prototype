port module Main exposing (..)

import Html
import Html.Attributes as Attribute
import Html.Events as Event
import WebSocket


main =
    Html.program
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


init : ( Model, Cmd Message )
init =
    ( { counter = 0, address = "ws://echo.websocket.org" }, Cmd.none )


type alias Model =
    { counter : Int
    , address : String
    }


type Message
    = Increment
    | Decrement
    | Receive String
    | Address String


update : Message -> Model -> ( Model, Cmd Message )
update message model =
    case message of
        Address address ->
            ( { model | address = address }, Cmd.none )

        Receive letter ->
            let
                value =
                    case String.toInt letter of
                        Ok v ->
                            v

                        Err _ ->
                            model.counter
            in
                ( { model | counter = value }, Cmd.none )

        Increment ->
            ( { model | counter = model.counter + 1 }, WebSocket.send model.address "increment" )

        Decrement ->
            let
                value =
                    max 0 (model.counter - 1)
            in
                ( { model | counter = value }, WebSocket.send model.address "decrement" )


view : Model -> Html.Html Message
view model =
    let
        allowed_to_decrement =
            model.counter > 0
    in
        Html.div []
            [ Html.button
                [ Attribute.disabled (not allowed_to_decrement)
                , Event.onClick Decrement
                ]
                [ Html.text "-" ]
            , Html.span [] [ Html.text (toString model.counter) ]
            , Html.button [ Event.onClick Increment ] [ Html.text "+" ]
            ]


port websocket_address : (String -> msg) -> Sub msg


subscriptions : Model -> Sub Message
subscriptions model =
    Sub.batch
        [ WebSocket.listen model.address
            Receive
        , websocket_address
            Address
        ]
