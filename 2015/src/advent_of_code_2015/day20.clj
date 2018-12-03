(ns advent-of-code-2015.day20
  (:require [clojure.string :as str]
            [clojure.set :as set]
            ))


(def input 33100000)
(def input-trimmed (quot 33100000 10))

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

(defn get-ans-2 [input]
  (->> (range)
       (filter (fn [num] (<= input (* 11 (apply + (filter #(>= 50 (quot num %)) (factors num)))))))
       (first)
       ))

(def ans1 (get-ans input-trimmed))
(def ans2 (get-ans-2 input))


