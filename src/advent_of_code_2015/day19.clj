(ns advent-of-code-2015.day19
  (:require [clojure.string :as str]
            [clojure.set :as set]
            ))


(def input (str/split (slurp "resources/day19") #"\n\n"))
(def formulas (->> (first input)
                   (#(str/split % #"\n"))
                   (map #(str/split % #" => "))
               ))
(def e-s (set (map second (re-seq #"e => (\w+)" (first input)))))
(def test-str (second input))

(defn replace-at-index [ss what with idx]
  (let [l (count what)
        st (subs ss 0 idx)
        end (subs ss (+ idx l))]
    (str st with end)))

(defn replacements
  ([ss what with] (replacements ss what with 0))
  ([ss what with idx]
   (let [next-idx (.indexOf ss what idx)]
     (case next-idx
       -1 nil
       (lazy-seq (cons (replace-at-index ss what with next-idx)
                 (replacements ss what with (inc next-idx))))))))

(defn get-replacements [ss formulas]
  (->> formulas
       (map (fn [[what with]] (replacements ss what with)))
       (map set)
       (reduce set/union)))

(defn make-step [strings formulas]
  (apply set/union (map #(get-replacements % formulas) strings)))

(defn calc-steps [e-s formulas test-str]
  (loop [strings e-s
         counter 1]
    (println counter)
    (let [expanded (make-step strings formulas)]
      (if (contains? expanded test-str)
        counter
        (recur expanded (inc counter))))))


(def ans1 (count (get-replacements test-str formulas)))
;; (def ans2 (calc-steps e-s formulas test-str))

