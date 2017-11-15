module Main exposing (..)

import Html
import Html.Attributes as Attribute
import Html.Events as Event


main =
    Html.program
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


init : (Model, Cmd Message)
init =
    ({ counter = 0 }, Cmd.none)


type alias Model =
    { counter : Int
    }


type Message
    = Increment
    | Decrement


update : Message -> Model -> ( Model, Cmd Message )
update message model =
    let
        next_model =
            case message of
                Increment ->
                    { model | counter = model.counter + 1 }

                Decrement ->
                    let
                        value =
                            max 0 (model.counter - 1)
                    in
                        { model | counter = value }
    in
        ( next_model, Cmd.none )


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


subscriptions : Model -> Sub Message
subscriptions _ =
    Sub.none
