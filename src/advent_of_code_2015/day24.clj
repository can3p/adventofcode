(ns advent-of-code-2015.day24
  (:require [clojure.string :as str]
            [clojure.math.combinatorics :as combo]
            ))

(def input (->> "resources/day24"
               (slurp)
               (#(str/split % #"\n"))
               (map #(Integer/parseInt %))
               ))

(def chunk-size (/ (apply + input) 3))
(def chunk-size-4 (/ (apply + input) 4))

(def chunks (->> input
                 (combo/subsets)
                 (filter #(= (apply + %) chunk-size))))

(def chunks-4 (->> input
                 (combo/subsets)
                 (filter #(= (apply + %) chunk-size-4))))

;; I bet this doesn'twork in general case
(def ans1 (apply * (first chunks)))

(def ans2 (apply * (first chunks-4)))

