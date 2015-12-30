(ns advent-of-code-2015.day22
  (:require [clojure.math.combinatorics :as combo]
            [clojure.set :as set]
            [clojure.string :as str]))

(def input (->> "resources/day22"
                (slurp)
                (re-seq #"\d+")
                (map #(Integer/parseInt %)))) ;; [hp damage]

(def init-mana 500)
(def init-hp 50)

(def spells #{ :missle :drain :shield :poison :recharge})
(def spell-defs {
                 :missle {
                          :cost 53
                          :fn (fn [state]
                                (assoc-in state [:boss :hp]
                                          (- (get-in state [:boss :hp]) 4)))
                          }
                 :drain {
                         :cost 73
                         :fn (fn [state]
                               (-> state
                                   (assoc-in [:boss :hp]
                                             (- (get-in state [:boss :hp]) 2))
                                   (assoc-in [:user :hp]
                                             (+ (get-in state [:user :hp]) 2))
                                   ))
                         }
                 :shield {
                          :cost 113
                          :turns 6
                          :fn (fn [state]
                                (assoc-in state [:user :armor] 7))
                          }

                 :poison {
                          :cost 173
                          :turns 6
                          :fn (fn [state]
                                (assoc-in state [:boss :hp]
                                          (- (get-in state [:boss :hp]) 3)))
                          }

                 :recharge {
                            :cost 229
                            :turns 5
                            :fn (fn [state]
                                  (assoc-in state [:user :mana]
                                            (+ (get-in state [:user :mana]) 101)))
                            }

                 })

(defn boss-turn [state]
  (if (or (= state true) (= state false)) state
      (if (< (get-in state [:user :hp]) 0) false
          (assoc-in state [:user :hp]
                    (- (get-in state [:user :hp]) (max 1 (- (get-in state [:boss :damage])
                                                            (get-in state [:user :armor])
                                                            )))))))

(defn fire-spell [state spell]
  ;; (println "user casts " spell)
  (if (get-in spell-defs [spell :turns])
    (assoc state :spells (conj (:spells state)
                               [spell (get-in spell-defs [spell :turns])]))
    ((get-in spell-defs [spell :fn]) state)
    )
  )

(defn user-turn [state spell]
  (if (or (= state true) (= state false)) state
      (if (> (get-in spell-defs [spell :cost]) (get-in state [:user :mana])) false
          (-> state
              (assoc-in [:user :mana]
                        (- (get-in state [:user :mana])
                           (get-in spell-defs [spell :cost])))
              (fire-spell spell)))))

(def init-state {
                 :user {
                        :hp init-hp
                        :mana init-mana
                        :armor 0
                        }
                 :boss {
                        :hp (first input)
                        :damage (second input)
                        }
                 :spells []
                 })

(defn process-spells [state]
  (if (or (= state true) (= state false)) state
      (if (= 0 (count (:spells state))) state
          (-> (reduce (fn [st [spell time]]
                        ;;but the fact that we reduce the time of the spell on the previous turn
                        ;; means that we should still execute spells that have zero
                        ;; days left and remove them only after that
                        (if (< time 0) st
                            ((get-in spell-defs [spell :fn]) st))) state (:spells state))
              ;; BAM!
              (#(assoc % :spells (filterv (fn [[_ time]] (<= 0 time)) (:spells %))))
              ))))

;; post effect is necessary, because we should choose spell after
;; counter decreased. In our case that means that we need to decrease
;; counters at the end of the previous turn, so that we can take
;; zeroed spells intro account on tree expansion
(defn post-process-spells [state]
  (if (or (= state true) (= state false)) state
      (if (= 0 (count (:spells state))) state
          (-> state
              (#(assoc % :spells (mapv (fn [[spell time]] [spell (dec time)]) (:spells %))))
              ))))

(defn kill-hp [state]
  (if (or (= state true) (= state false)) state
      (let [new-st (assoc-in state [:user :hp]
                             (- (get-in state [:user :hp]) 1))]
        (if (<= (get-in new-st [:user :hp]) 0)
          false
          new-st))))

(defn exec-turn [state turn hard-mode]
  (cond
    (= state true) true
    (= state false) false
    :else
    (-> state
        #_(#(do (println "before" % turn) %))
        (#(if (or (not hard-mode) (:boss turn)) %
                  (kill-hp state)))
        ;; (#(do (println "after" % turn) %))
        process-spells
        (#(if (and (:boss %) (<= (get-in % [:boss :hp]) 0)) true %))
        ;; (#(do (println "before" %) %))
        (#(if (:boss turn)
            (boss-turn %)
            (user-turn % (:spell turn))))
        (#(cond
            (= % true) true
            (= % false) false
            (<= (get-in % [:boss :hp]) 0) true
            (<= (get-in % [:user :hp]) 0) false
            :else
            (assoc-in % [:user :armor] 0)
            ))
        post-process-spells
        #_(#(do (println "after" %) %))
        )))

(defn exec-turn-boss-loop [state spell hard-mode]
  (-> state
      (exec-turn {:spell spell} hard-mode)
      (exec-turn {:boss true} hard-mode)
      ))

(defn get-available [st]
  (let [used-spells (set (map first (filterv (fn [[_ cost]] (> cost 0)) (:spells st))))]
    ;; (println used-spells st)
    (sort-by #(get-in spell-defs [% :cost]) (set/difference spells used-spells))))

(defn spell-cost [spell]
  (get-in spell-defs [spell :cost]))

(defn find-min-win-path [state spent cut-off hard-mode]
  (if (>= spent cut-off) nil
      (loop [av (if (= 0 spent) (list :poison) (get-available state))
             cur-cut-off cut-off
             cur-win-path nil]
        (if (= 0 (count av))
          (do
            (list cur-cut-off cur-win-path)
            )
          (let [spell (first av)
                cost (spell-cost spell)
                will-spend (+ cost spent)
                tries (rest av)]
            (if (> will-spend cur-cut-off)
              (recur tries cur-cut-off cur-win-path)
              (let [outcome (exec-turn-boss-loop state spell hard-mode)]
                (cond
                  (= outcome true) (recur tries will-spend (list spell))
                  (= outcome false) (recur tries cur-cut-off cur-win-path)
                  :else (let [[new-cut-off path] (find-min-win-path outcome will-spend cur-cut-off hard-mode)]
                          (if path
                            (do
                              (recur tries new-cut-off (conj path spell))
                              )
                            (recur tries cur-cut-off cur-win-path)
                            ))))))))))

(def ans1 (first (find-min-win-path init-state 0 1e6 false)))
(def ans2 (first (find-min-win-path init-state 0 1e6 true)))
