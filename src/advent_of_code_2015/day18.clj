(ns advent-of-code-2015.day18
  (:require [clojure.string :as str]
            [clojure.set :as set]
            ))


(def input (slurp "resources/day18"))
(def grid-size 100)
(def steps 100)

(def ref-input ".#.#.#
...##.
#....#
..#...
#.#..#
####..")

(defn process-input [input]
  (->> input
       (#(str/replace % "\n" ""))
       (seq)
       (vec)
       (mapv #(case %
               \. 0
               \# 1))))

(defn deviations [x y]
  (list
   [(dec x) (dec y)]
   [(dec x) y]
   [(dec x) (inc y)]

   [(inc x) (dec y)]
   [(inc x) y]
   [(inc x) (inc y)]

   [x (dec y)]
   [x (inc y)]
        ))

(defn get-neighbours [idx size]
  (let [x (mod idx size)
        y (quot idx size)]

    (->> (deviations x y)
         (filter (fn [[x y]] (and
                             (< x size)
                             (< y size)
                             (>= x 0)
                             (>= y 0)
                             )))
         (map (fn [[x y]]
                (+ x (* y size))))
         )))

(def get-neightbours-memo (memoize get-neighbours))

(defn get-next-state [grid size idx]
  (let [val (get grid idx)
        neigh (get-neightbours-memo idx size)
        num-on (reduce + (map #(get grid %) neigh))
        ]
    (cond
      (and (= 1 val) (or (= num-on 2) (= num-on 3))) 1
      (= 1 val) 0
      (and (= 0 val) (= num-on 3)) 1
      (= 0 val) 0)))

(defn get-next-state-crn [grid size idx]
  (let [val (get grid idx)
        neigh (get-neightbours-memo idx size)
        num-on (reduce + (map #(get grid %) neigh))
        ]
    (cond
      (= idx 0) 1
      (= idx (dec size)) 1
      (= idx (dec (* size size))) 1
      (= idx (* size (dec size))) 1
      (and (= 1 val) (or (= num-on 2) (= num-on 3))) 1
      (= 1 val) 0
      (and (= 0 val) (= num-on 3)) 1
      (= 0 val) 0)))

(defn get-next-grid [grid size func]
  (vec (map-indexed (fn [idx _]
                      (func grid size idx)) grid)))

(defn fix-bulbs [grid size]
  (vec (map-indexed (fn [idx v]
                 (cond
                   (= idx 0) 1
                   (= idx (dec size)) 1
                   (= idx (dec (* size size))) 1
                   (= idx (* size (dec size))) 1
                   :default v
                   )) grid)))


(def ans1 (reduce +
                  (first
                   (drop steps (iterate #(get-next-grid % grid-size get-next-state) (process-input input))))))

(def ans2 (reduce +
                  (first
                   (drop steps (iterate #(get-next-grid % grid-size get-next-state-crn) (fix-bulbs (process-input input) grid-size))))))
