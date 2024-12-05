module Main (main) where

import Hue.Lowering (Bytecode (..), Function (..), Instruction (..), lower)
import Hue.Parser (Node (..), ParsingError (..), parse)
import Hue.Typechecker (Type (..), TypedNode (..), typecheck)
import Test.Hspec (SpecWith, describe, hspec, it, shouldBe)

main :: IO ()
main = hspec $ do
  hueParser
  hueTypechecker
  hueLowering

hueParser :: SpecWith ()
hueParser =
  describe "Hue.Parser" $ do
    it "errors on empty input" $ do
      let got = parse ""
       in let expected = Left (ParsingError "expected symbol" "")
           in got `shouldBe` expected

    it "errors on remaining input" $ do
      let got = parse "main = \"hello world\" remaining input"
       in let expected = Left (ParsingError "remaining input" "remaining input")
           in got `shouldBe` expected

    it "errors on malformed input" $ do
      let got = parse "main ="
       in let expected = Left (ParsingError "expected string" "")
           in got `shouldBe` expected

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

hueLowering :: SpecWith ()
hueLowering =
  describe "Hue.Lowering" $ do
    it "lowers binding" $ do
      let got = lower $ TypedBind (TypedSymbol TypeString "main") (TypedString "hello world")
       in let expected = Bytecode [Function "main" [LoadString "hello world", Return]]
           in got `shouldBe` expected
