module NestedParens (
  Parens,
  makeParens,
  nextParens,
  ) where

import Data.Word (Word64)
import Data.Bits ((.&.), (.|.), xor, shiftL, shiftR,
  complement, popCount)

-- Problem 7.1.3.23 of TAOCP 4A

-- Holds parenthesis as 0s for left (, 1s for ),
--  bitmask to identify valid bits
data Parens = Parens Word64 Word64

instance Show Parens where
  show (Parens state endmask) = showAccum state endmask []

showAccum :: Word64 -> Word64 -> [Char] -> String
showAccum currState currMask accum | odd currMask = accum
showAccum currState currMask accum | odd currState =
  showAccum (currState `shiftR` 1) (currMask `shiftR` 1) (')':accum)
showAccum currState currMask accum =
  showAccum (currState `shiftR` 1) (currMask `shiftR` 1) ('(':accum)

mu0 :: Word64
mu0 = 0x5555555555555555 :: Word64

-- Make a parens in it's initial state
makeParens :: Int -> Parens
makeParens n | n > 64 = error "Invalid (>64) argument"
makeParens n | n <= 0 = error "Invalid (non-positive) argument"
makeParens n | odd n = error "Invalid (non-even) argument"
makeParens n = Parens (makeInitial n) (makeMask n)

-- Make initial state
makeInitial :: Int -> Word64
makeInitial n = 2^(n `div` 2) - 1

-- Make mask
makeMask :: Int -> Word64
makeMask n = -1 `shiftL` n

-- See if the parens state is valid
isValid :: Parens -> Bool
isValid (Parens state mask) = (state .&. mask) == 0

-- Get next state, Nothing if pattern is exhausted
nextParens :: Parens -> Maybe Parens
nextParens (Parens state mask) =
  let t = state `xor` mu0
      u = t `xor` (t - 1)
      v = state .|. u
      w = v + 1
      s = popCount (u .&. mu0)
      wp = (v .&. (complement w)) `shiftR` s
      newP = Parens (w + wp) mask
  in if (isValid newP) then Just newP else Just Nothing

