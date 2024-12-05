module Hue.Typechecker where

import Hue.Parser (Node (..))

data Type
  = TypeUnknown
  | TypeString
  deriving (Show, Eq)

data TypedNode
  = TypedSymbol Type String
  | TypedString String
  | TypedBind TypedNode TypedNode
  deriving (Show, Eq)

data TypecheckingError = CannotUnify Type Type deriving (Show, Eq)

typecheck :: Node -> Either TypecheckingError TypedNode
typecheck node = do
  checkedNode <- check TypeUnknown node
  Right checkedNode

unify :: Type -> Type -> Either TypecheckingError Type
unify TypeUnknown rhs = Right rhs
unify lhs TypeUnknown = Right lhs
unify lhs rhs =
  if lhs /= rhs
    then
      Left $ CannotUnify lhs rhs
    else
      Right lhs

check :: Type -> Node -> Either TypecheckingError TypedNode
check _ (NodeString value) = Right $ TypedString value
check _ (NodeBind symbol value) =
  let inferredType = infer value
   in do
        checkedSymbol <- check inferredType symbol
        checkedValue <- check inferredType value
        Right $ TypedBind checkedSymbol checkedValue
check ty (NodeSymbol value) =
  let inferredType = infer (NodeSymbol value)
   in do
        unifiedType <- unify ty inferredType
        Right $ TypedSymbol unifiedType value

infer :: Node -> Type
infer (NodeSymbol _) = TypeUnknown
infer (NodeString _) = TypeString
infer (NodeBind _ value) = infer value

typeOf :: TypedNode -> Type
typeOf (TypedSymbol ty _) = ty
typeOf (TypedString _) = TypeString
typeOf (TypedBind symbol _) = typeOf symbol
