module Main (main) where

import Hue.Parser (Node (..), parse)
import Test.Hspec (describe, hspec, it, shouldBe)

main :: IO ()
main = hspec $ do
  describe "Hue.Parser" $ do
    it "parses binding" $ do
      let got = parse "main = \"hello world\""
       in let expected = Right $ (NodeBind (NodeSymbol "main") (NodeString "hello world"), "")
           in got `shouldBe` expected
