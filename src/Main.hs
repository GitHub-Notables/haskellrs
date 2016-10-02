import FLib
import Foreign.C.String (newCString)

main :: IO ()
main = do
  putStrLn $ show $ doubleInput 3
  str <- newCString "Hello World\0"
  printString str
