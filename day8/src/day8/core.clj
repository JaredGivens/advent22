(ns day8.core
  (:gen-class))

(use '[clojure.string :only [index-of]])


(defn -main []
  (def file (slurp "input"))
  (def row_len (inc(index-of file "\n")))

  (def count-series (fn [i0 h0 step in-bounds found]
    (let [i1 (+ i0 step) 
          h1 (int (get file i0))]
      (if (in-bounds i1)
        (if (and (< h0 h1))
          (count-series i1 h1 step in-bounds (conj found i0))
          (count-series i1 h0 step in-bounds found)
        ) 
        found
      )
    )
  ))

  (def file_len (count file))

  (def iter-rows (fn [cur_ind found]
    (let [nex_ind (+ cur_ind row_len)]
      (if (< cur_ind file_len)
        (iter-rows nex_ind 
          (count-series (+ nex_ind -2) 0 -1 (fn [i] (> i (dec cur_ind)))
            (count-series cur_ind 0 1 (fn [i] (< i (dec nex_ind))) found)
          )
        )
        found
      )
    )
  ))

  (def iter-cols (fn [i0 found]
    (let [i1 (inc i0)]
      (if (< i0 (dec row_len))
        (iter-cols i1
          (count-series (- file_len i0 2) 0 (* -1 row_len) (fn [i] (> i -1))
            (count-series i0 0 row_len (fn [i] (< i file_len)) found) 
          )
        )
        found
      )
    )
  ))

  (def found #{0})

  (println (count (iter-cols 0 (iter-rows 0 found))))
)
