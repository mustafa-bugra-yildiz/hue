module Main (main) where

import Hue.Parser (Node (..), parse)
import Hue.Typechecker (Type (..), TypedNode (..), typecheck)
import Test.Hspec (SpecWith, describe, hspec, it, shouldBe)

main :: IO ()
main = hspec $ do
  hueParser
  hueTypechecker

hueParser :: SpecWith ()
hueParser =
  describe "Hue.Parser" $ do
    it "parses binding" $ do
      let got = parse "main = \"hello world\""
       in let expected = Right $ NodeBind (NodeSymbol "main") (NodeString "hello world")
           in got `shouldBe` expected

hueTypechecker :: SpecWith ()
hueTypechecker =
  describe "Hue.Typechecker" $ do
    it "typecheckes binding" $ do
      let got = typecheck $ NodeBind (NodeSymbol "main") (NodeString "hello world")
       in let expected = Right $ TypedBind (TypedSymbol TypeString "main") (TypedString "hello world")
           in got `shouldBe` expected
