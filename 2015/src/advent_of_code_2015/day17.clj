(ns advent-of-code-2015.day17
  (:require [clojure.string :as str]
            [clojure.set :as set]
            [clojure.math.combinatorics :as combo]
            ))


(def input (slurp "resources/day17"))
(def liters 150)

(defn parse-input [input]
  (map (fn [& v] (vec v))
       ;; add index to trick combo/subsets, because it doesn't include duplicates otherwise
       (iterate inc 1)
       (map #(Integer/parseInt %)
            (str/split input #"\n"))))

(defn find-combs [input liters]
  (count (filter #(= liters (reduce (fn [acc [c l]] (+ acc l)) 0 %))
                 (rest (combo/subsets (parse-input input))))))

(defn find-min-combs [input liters]
  (let [combs (filter #(= liters (reduce (fn [acc [c l]] (+ acc l)) 0 %))
                      (rest (combo/subsets (parse-input input))))
        counts (map count combs)
        min-count (reduce min counts)]
    (count (filter #(= % min-count) counts))))


(def ans1 (find-combs input liters))
(def ans2 (find-min-combs input liters))

