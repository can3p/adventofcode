(ns advent-of-code-2015.day10
  (:require [clojure.string :as str]
            [clojure.set :as set]))

(def input "3113322113")
(def iterations 40)
(def iterations-2 50)

(defn group [input]
  (loop [
         chrs (seq input)
         cur nil
         cur-group []
         groups []
         ]
    (if (= 0 (count chrs)) (conj groups cur-group)
        (let [f (first chrs)
              r (rest chrs)]
          (if (or (= nil cur) (= f cur))
            (recur r
                   f
                   (conj cur-group f)
                   groups
                   )
            (recur r
                   f
                   [f]
                   (conj groups cur-group)
                   )
            )))))


(defn ite [input]
  (let [g (group input)]
    (apply str (map #(str (count %) (first %)) g)
            )))

(defn do-times [input times]
  (if (= 0 times) input
      (do-times (ite input) (dec times))
      ))

(def ans1 (count (do-times input iterations)))
(def ans2 (count (do-times input iterations-2)))

