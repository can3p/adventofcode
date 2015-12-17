(ns advent-of-code-2015.day9
  (:require [clojure.string :as str]
            [clojure.set :as set]))

(def input "AlphaCentauri to Snowdin = 66
AlphaCentauri to Tambi = 28
AlphaCentauri to Faerun = 60
AlphaCentauri to Norrath = 34
AlphaCentauri to Straylight = 34
AlphaCentauri to Tristram = 3
AlphaCentauri to Arbre = 108
Snowdin to Tambi = 22
Snowdin to Faerun = 12
Snowdin to Norrath = 91
Snowdin to Straylight = 121
Snowdin to Tristram = 111
Snowdin to Arbre = 71
Tambi to Faerun = 39
Tambi to Norrath = 113
Tambi to Straylight = 130
Tambi to Tristram = 35
Tambi to Arbre = 40
Faerun to Norrath = 63
Faerun to Straylight = 21
Faerun to Tristram = 57
Faerun to Arbre = 83
Norrath to Straylight = 9
Norrath to Tristram = 50
Norrath to Arbre = 60
Straylight to Tristram = 27
Straylight to Arbre = 81
Tristram to Arbre = 90")

(defn parse-connections [input]
  (reduce (fn [[connections points] s]
            (let [[from _ to _ dist] (str/split s #"\s+")]
              [
               (assoc connections #{from to} (Integer/parseInt dist))
               (conj points from to)
               ]))
          [{} #{}]
          (str/split input #"\n")))

(defn expand-points
  ([points]
   (map (fn [point] {
                     :current point
                     :left (disj points point)
                     :dist 0
                     })
        points))

  ([points current dist connections]
   (map (fn [point] {
                     :current point
                     :left (disj points point)
                     :dist (+ dist (connections #{current point}))
                     })
        points)))

(defn find-min [state]
  (let [sr-state (sort-by :dist state)]
    [(first sr-state) (rest sr-state)]))

(defn find-max [state]
  (let [sr-state (reverse (sort-by :dist state))]
    [(first sr-state) (rest sr-state)]))


(defn victory? [st] (= 0
                       (count (:left st))))

(defn solve [input cmp]
  (let [[connections points] (parse-connections input)]
    (loop [states (expand-points points)]
      (let [[min-state r-states] (cmp states)]
        (if (victory? min-state)
          (:dist min-state)
          (recur (concat r-states
                         (expand-points (:left min-state)
                                        (:current min-state)
                                        (:dist min-state)
                                        connections)))
          )))))

(def ans1 (solve input find-min))
(def ans2 (solve input find-max))
