(ns advent-of-code-2015.day14
  (:require [clojure.string :as str]
            ))

(def input (slurp "resources/day14"))

(defn process-str [str]
  (let [[_ speed go stay] (re-find #"[^\d]+(\d+)[^\d]+(\d+)[^\d]+(\d+)" str)]
    (cycle (concat
            (repeat (Integer/parseInt go) (Integer/parseInt speed))
            (repeat (Integer/parseInt stay) 0)))))


(defn calc-fastest [input time]
  (reduce max (map #(reduce + %)
                   (map #(take time %)
                        (map process-str (str/split input #"\n"))))))

;; map/reduce/apply hell
(defn calc-fastest-new [input time]
  (apply max
         (reduce (fn [nums1 nums2]
                   (map + nums1 nums2))
                 (apply map (fn [& nums]
                              (let [m (apply max nums)]
                                (map (fn [n] (if (= n m) 1 0))
                                     nums)))
                        (map #(reductions + %)
                             (map #(take time %)
                                  (map process-str (str/split input #"\n"))))))))

(def ans1 (calc-fastest input 2503))
(def ans2 (calc-fastest-new input 2503))
