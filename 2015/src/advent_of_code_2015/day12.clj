(ns advent-of-code-2015.day12
  (:require [clojure.string :as str]
            [clojure.set :as set]
            [clojure.data.json :as json]
            [clojure.walk :as walk]
            ))

(def input (json/read-str (slurp "resources/day12.json")))

(defn has-red [h]
  (->> h
       (group-by val)   ; sorted into a new map based on value of each key
       (#(get % "red"))
       some?
       ))

(defn to-seq [exc-red js]
  (->> js
       (walk/postwalk (fn [item]
                        (cond
                          (and exc-red (map? item) (has-red item)) nil
                          (map? item) (seq item)
                          :default item
                          )))
       (flatten)
       ))

(defn calc-nums [input exc-red]
  (->> input
       (to-seq exc-red)
       (filter number?)
       (reduce +)))

(def ans1 (calc-nums input false))
(def ans2 (calc-nums input true))
