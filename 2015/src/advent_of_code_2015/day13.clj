(ns advent-of-code-2015.day13
  (:require [clojure.string :as str]
            [clojure.set :as set]
            [clojure.math.combinatorics :as combo]
            ))

(def input (slurp "resources/day13"))

(defn process-str [str]
  (let [[_ a key sc b] (re-find #"(\w+).*(gain|lose) (\d+).*\s(\w+).$" str)
        score (if (= key "gain") (Integer/parseInt sc) (- (Integer/parseInt sc)))]
    (list #{a b} score)))


(defn process-input [input]
  (reduce (fn [[scores guests] [pair score]]
            (list
             (assoc scores pair (+ score (if (nil? (scores pair)) 0 (scores pair))))
             (set/union guests pair)
             ))
          (list {} #{})
          (map process-str (str/split input #"\n"))))

(defn calc-score [scores seats]
  (reduce +
          (map #(get scores (set %))
               (partition 2 1
                          (conj (seq seats) (last (seq seats))))
               )))

(defn scores-seq [input]
  (let [[scores guests] (process-input input)]
    (map #(calc-score scores %)
         (combo/permutations guests))))

(defn add-you [scores guests]
  (let [you "You"
        g1 (conj guests you)
        sc (apply hash-map (flatten (map (fn [a b] (list #{a b} 0))
                                   guests
                                   (repeat you)
                                   )))]
    (list (merge scores sc) g1)))

(defn scores-seq-you [input]
  (let [[scores guests] (apply add-you (process-input input))]
    (map #(calc-score scores %)
         (combo/permutations guests))))

(def ans1 (reduce max (scores-seq input)))
(def ans2 (reduce max (scores-seq-you input)))
