(ns advent-of-code-2015.day23
  (:require [clojure.string :as str]
            [clojure.math.combinatorics :as combo]
            ))

(def input (slurp "resources/day23"))

(def ref-input "inc a
jio a, +2
tpl a
inc a")

(defn power [base n]
  (case n
    1 base
    (* base (power base (dec n)))))


(defn process-instruction [instructions state]
  (let [idx (:idx state)
        next-idx (inc idx)
        [cmd buf param] (re-seq #"[-\w]+" (get instructions idx))
        iparam (cond
                 (= cmd "jmp") (Integer/parseInt buf)
                 param (Integer/parseInt param)
                 :else nil
                 )
        ]
    ;; (println cmd buf param (get state "a"))
    (case cmd
      "inc" (assoc state buf (inc (get state buf))
                   :idx next-idx)
      "tpl" (assoc state buf (* (get state buf) 3)
                   :idx next-idx)
      "hlf" (assoc state buf (/ (get state buf) 2)
                   :idx next-idx)
      "jmp" (assoc state :idx (+ idx iparam))
      "jie" (assoc state :idx (if (even? (get state buf)) (+ idx iparam) next-idx))
      "jio" (assoc state :idx (if (= 1 (get state buf)) (+ idx iparam) next-idx))
      (throw "invalid command")
      )))


(defn execute [input a-start]
  (let [instructions (vec (str/split input #"\n"))]
    (loop [state {
                  "a" (bigint a-start)
                  "b" (bigint 0)
                  :idx 0
                  }]
      (if (>= (:idx state) (count instructions))
        state
        (recur (process-instruction instructions state))))))

(def ans1 (execute input 0))
(def ans2 (execute input 1))
