(ns advent-of-code-2015.day19
  (:require [clojure.string :as str]
            [clojure.set :as set]
            ))


(def input (str/split (slurp "resources/day19") #"\n\n"))
(def formulas (->> (first input)
                   (#(str/split % #"\n"))
                   (map #(str/split % #" => "))
               ))

(def r-formulas (map (comp vec reverse) formulas))
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

(defn find-matches [test-str r-formulas]
  (sort-by (comp count first) > (filter #(<= 0 (.indexOf test-str (first %))) r-formulas)))

(defn calc-matches [matches r-formulas target-str cut-off counter]
  (loop [m matches
         cur-cut-off cut-off
         found-steps -1]
    (if (= 0 (count m)) found-steps
        (let [found (calc-steps r-formulas
                                (first m)
                                target-str
                                cur-cut-off
                                (inc counter)
                                )]
          (if (not= -1 found) found
              (recur (rest m)
                     (if (= -1 found) cur-cut-off found)
                     (if (= -1 found) found-steps found)
                     )))

        )))

(defn calc-steps [r-formulas test-str target-str cut-off counter]
  (cond
    (= cut-off counter) -1
    (> (count target-str) (count test-str)) -1
    (= target-str test-str) counter
    :else (let [matches (->> test-str
                             (#(find-matches % r-formulas))
                             (map (fn[[a b]] (str/replace-first test-str a b))))
                             ]
            (calc-matches matches r-formulas target-str cut-off counter)
            )))


(def ans1 (count (get-replacements test-str formulas)))
(def ans2 (calc-steps r-formulas test-str "e" 400 0))
