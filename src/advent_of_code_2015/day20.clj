(ns advent-of-code-2015.day20
  (:require [clojure.string :as str]
            [clojure.set :as set]
            ))


(def input (quot 33100000 10))

(defn factors [n]
  (into (sorted-set)
        (reduce concat
                (for [x (range 1 (inc (Math/sqrt n))) :when (zero? (rem n x))]
                  [x (/ n x)]))))

(defn get-ans [input]
  (->> (range)
       (filter (fn [num] (<= input (apply + (factors num)))))
       (first)
       ))

(def ans1 (get-ans input))


