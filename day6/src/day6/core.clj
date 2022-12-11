(ns day6.core
  (:gen-class))

(use '[clojure.string :only [index-of]])

(def ans (atom #{}))
(def file (slurp "input"))
(def marker (re-find #"((\w)((?!\2)\w)((?!\2)(?!\3)\w)((?!\2)(?!\3)(?!\4)\w))" file))

(defn compute-ans [i n]
  (if-not (zero? n)
    (do
      (swap! ans conj (get file (- i n)))
      (compute-ans i (dec n))
    )
  )
)

(defn find-marker [i amt]
  (reset! ans #{})
  (if (< i (count file))

    (if (>= i amt)
      (do
        (compute-ans i amt)
        (println (count @ans))
        (if (= (count @ans) amt)
          i
          (find-marker (inc i) amt)
        )
      )
      (find-marker (inc i) amt)
    )
  )
)

(defn -main []
  (println (inc(find-marker 0 14)))

  (println marker)
  (println (count file))
  (println (index-of file (get marker 0)))
)