(ns advent-of-code-2015.day25
  (:require [clojure.string :as str]
            [clojure.math.combinatorics :as combo]
            ))

(def input (->> "resources/day25"
                (slurp)
                (re-seq #"\d+")
                (map #(Integer/parseInt %)) ;; [row col] => [ y x ]
                (reverse))) ;; [ x y ]

(defn sum-prog [n]
  (* n (/ (+ 1 n) 2)))

(defn iter [x]
  (mod (* 252533 x) 33554393))

(defn get-code [x y]
  (+ x (sum-prog (+ x y -2))))

(def codes (iterate iter 20151125))

(def ans1 (first (drop (dec (apply get-code input)) codes)))


