(ns advent-of-code-2015.day16
  (:require [clojure.string :as str]
            [clojure.set :as set]
            ))

(def input (slurp "resources/day16"))
(def analysis {
               "children" 3
               "cats" 7
               "samoyeds" 2
               "pomeranians" 3
               "akitas" 0
               "vizslas" 0
               "goldfish" 5
               "trees" 3
               "cars" 2
               "perfumes" 1
               })

(defn parse-str [s]
  (let [[_ num props] (re-find #"Sue (\d+): (.*)" s)]
    (list num (apply hash-map (map #(if (re-find #"\d" %) (Integer/parseInt %) %)
                                   (str/split props #"(,|:) "))))))

(defn includes? [hash1 hash2]
  (let [k (keys hash2)]
    (reduce #(and %1 %2)
            (map #(= (get hash1 %) (get hash2 %)) k))))

(defn strange-matcher [hash1 hash2]
  (let [k (keys hash2)]
    (reduce #(and %1 %2)
            (map #(and (get hash1 %)
                       (case %
                         "cats" (< (get hash1 %) (get hash2 %))
                         "trees" (< (get hash1 %) (get hash2 %))
                         "pomeranians" (> (get hash1 %) (get hash2 %))
                         "goldfish" (> (get hash1 %) (get hash2 %))
                         (= (get hash1 %) (get hash2 %))
                         )) k))))

(defn find-matches [input match matcher]
  (filter (fn [[num props]] (matcher match props))
          (map parse-str
               (str/split input #"\n"))))

(def ans1 (first (first (find-matches input analysis includes?))))
(def ans2 (first (first (find-matches input analysis strange-matcher))))

