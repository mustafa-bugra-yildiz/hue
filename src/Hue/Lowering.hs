module Hue.Lowering where

import Data.List (intercalate)
import Hue.Typechecker (TypedNode (TypedBind, TypedString, TypedSymbol))

newtype Bytecode = Bytecode [Function] deriving (Eq)

instance Show Bytecode where
  show (Bytecode functions) = intercalate "\n" (map show functions)

data Function = Function String [Instruction] deriving (Eq)

instance Show Function where
  show (Function name instructions) =
    ".text\n" ++ name ++ ":\n" ++ intercalate "\n" (map (\x -> " " ++ show x) instructions)

data Instruction = LoadString String | Return deriving (Eq)

instance Show Instruction where
  show (LoadString value) = "adr x0, \"" ++ value ++ "\""
  show Return = "ret"

lower :: TypedNode -> Bytecode
lower = lowerBytecode (Bytecode [])

lowerBytecode :: Bytecode -> TypedNode -> Bytecode
lowerBytecode (Bytecode fns) (TypedBind symbol value) = Bytecode (lowerFunction symbol value : fns)
lowerBytecode _ _ = error "unreachable"

lowerFunction :: TypedNode -> TypedNode -> Function
lowerFunction (TypedSymbol _ name) value = Function name (lowerExpr value ++ [Return])
lowerFunction _ _ = error "unreachable"

lowerExpr :: TypedNode -> [Instruction]
lowerExpr (TypedString value) = [LoadString value]
lowerExpr _ = error "unreachable"
