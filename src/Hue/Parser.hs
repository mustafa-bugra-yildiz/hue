module Hue.Parser where

import Data.Char (isAlpha, isSpace)
import Data.List (isPrefixOf)
import Hue.Either (mapLeft)

data Node
  = NodeSymbol String
  | NodeString String
  | NodeBind Node Node
  deriving (Show, Eq)

-- ParsingError (Message) (Remaining Input)
data ParsingError
  = ParsingError String String
  deriving (Show, Eq)

withMessage :: String -> ParsingError -> ParsingError
withMessage msg (ParsingError _ i) = ParsingError msg i

parse :: String -> Either ParsingError Node
parse input = do
  (symbol, afterSymbol) <- parseSymbol input
  afterEq <- parseTag "=" afterSymbol
  (string, afterString) <- parseString afterEq
  let remaining = ws afterString
   in if remaining /= ""
        then
          Left $ ParsingError "remaining input" remaining
        else
          Right $ NodeBind symbol string

parseString :: [Char] -> Either ParsingError (Node, [Char])
parseString input = do
  afterLhsQuote <- mapLeft (withMessage "expected string") $ parseTag "\"" i
  let value = takeWhile (/= '\"') afterLhsQuote
   in let afterValue = drop (length value) afterLhsQuote
       in do
            afterRhsQuot <- parseTag "\"" afterValue
            Right (NodeString value, afterRhsQuot)
  where
    i = ws input

parseSymbol :: [Char] -> Either ParsingError (Node, [Char])
parseSymbol input =
  case length value of
    0 ->
      Left $ ParsingError "expected symbol" i
    _ ->
      Right (NodeSymbol value, rest)
  where
    i = ws input
    value = takeWhile isAlpha i
    rest = drop (length value) i

parseTag :: String -> [Char] -> Either ParsingError [Char]
parseTag tag input =
  if value
    then
      Right rest
    else
      Left $ ParsingError ("expected tag \"" ++ tag ++ "\"") i
  where
    i = ws input
    value = tag `isPrefixOf` i
    rest = drop (length tag) i

ws :: [Char] -> [Char]
ws = dropWhile isSpace
