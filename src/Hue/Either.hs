module Hue.Either where

mapLeft :: (a -> c) -> Either a b -> Either c b
mapLeft f = mapBoth f id

mapBoth :: (a -> c) -> (b -> d) -> Either a b -> Either c d
mapBoth f _ (Left x) = Left (f x)
mapBoth _ f (Right x) = Right (f x)
