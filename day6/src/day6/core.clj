(ns day6.core
  (:gen-class))

(use '[clojure.string :only [index-of]])

(defn -main []
  (def file (slurp "input"))

  (def marker (re-find #"((\w)((?!\2)\w)((?!\2)(?!\3)\w)((?!\2)(?!\3)(?!\4)\w))" file))
  (println marker)
  (println (index-of file (get marker 0)))
)