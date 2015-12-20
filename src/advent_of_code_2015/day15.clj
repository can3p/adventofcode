(ns advent-of-code-2015.day15
  (:require [clojure.string :as str]
            [clojure.set :as set]
            [clojure.math.combinatorics :as combo]
            ))

(def input (slurp "resources/day15"))
(def sum-num 100)
(def calories 500)

(def ref-input "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3")

(defn process-str [str]
  (let [[_ & params] (re-find #"[^-\d]+([-\d]+)[^-\d]+([-\d]+)[^-\d]+([-\d]+)[^-\d]+([-\d]+)[^-\d]+([-\d]+)" str)]
    (vec (map #(Integer/parseInt %) params))))

(defn process-input [input]
  (map process-str (str/split input #"\n")))

(defn mulv [v m]
  (mapv * v (repeat m)))

(defn calc-score [coefs weights]
  (let [[a b c d e] (map #(if (< % 0) 0 %)
                         (apply map +
                                (map mulv coefs weights)))]
    (list (* a b c d) e)))

(defn get-combinations [max-num cols]
  (filter #(= max-num (reduce + %)) 
          (combo/selections (range 0 (inc max-num)) cols)))

(defn get-highest [input sum-num]
  (let [coefs (process-input input)
        coef-num (count coefs)]
    (reduce max
            (map first
                 (map #(calc-score coefs %)
                      (get-combinations sum-num coef-num))))))

(defn get-highest-with-calories [input sum-num calories]
  (let [coefs (process-input input)
        coef-num (count coefs)]
    (reduce max
            (map first
                 (filter (fn [[a b]] (= calories b))
                         (map #(calc-score coefs %)
                              (get-combinations sum-num coef-num)))))))

(def ans1 (get-highest input sum-num))
(def ans2 (get-highest-with-calories input sum-num calories))
