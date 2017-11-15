module Main exposing (..)

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
    ( { counter = 0, lastLetter = Nothing, address = "ws://echo.websocket.org" }, Cmd.none )


type alias Model =
    { counter : Int
    , lastLetter : Maybe String
    , address : String
    }


type Message
    = Increment
    | Decrement
    | Receive String
    | Send


update : Message -> Model -> ( Model, Cmd Message )
update message model =
    case message of
        Receive letter ->
            ( { model | lastLetter = Just letter }, Cmd.none )

        Send ->
            ( model, WebSocket.send model.address "test" )

        Increment ->
            ( { model | counter = model.counter + 1 }, Cmd.none )

        Decrement ->
            let
                value =
                    max 0 (model.counter - 1)
            in
                ( { model | counter = value }, Cmd.none )


view : Model -> Html.Html Message
view model =
    let
        allowed_to_decrement =
            model.counter > 0

        maybe_letter =
            case model.lastLetter of
                Just letter ->
                    letter

                Nothing ->
                    ""
    in
        Html.div []
            [ Html.button
                [ Attribute.disabled (not allowed_to_decrement)
                , Event.onClick Decrement
                ]
                [ Html.text "-" ]
            , Html.span [] [ Html.text (toString model.counter) ]
            , Html.button [ Event.onClick Increment ] [ Html.text "+" ]
            , Html.span [] [ Html.text maybe_letter ]
            , Html.button [ Event.onClick Send ] [ Html.text "send" ]
            ]


subscriptions : Model -> Sub Message
subscriptions model =
    WebSocket.listen model.address Receive
