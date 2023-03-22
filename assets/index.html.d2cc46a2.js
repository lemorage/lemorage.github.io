import{_ as a,o as n,c as e,d as s}from"./app.afd00e04.js";const i={},t=s(`<h2 id="q1-distribute-money-to-maximum-children" tabindex="-1"><a class="header-anchor" href="#q1-distribute-money-to-maximum-children" aria-hidden="true">#</a> Q1 Distribute Money to Maximum Children</h2><h3 id="solution-1" tabindex="-1"><a class="header-anchor" href="#solution-1" aria-hidden="true">#</a> Solution 1</h3><h4 id="intuition" tabindex="-1"><a class="header-anchor" href="#intuition" aria-hidden="true">#</a> Intuition</h4><p>Brute-force.</p><h4 id="approach" tabindex="-1"><a class="header-anchor" href="#approach" aria-hidden="true">#</a> Approach</h4><ul><li>If the money is less than the children, then return -1.</li><li>Otherwise, we distribute each children by 1 dollar to satisfy the second condition. <ul><li>We then use a while loop to check how many children can receive 7 dollars each, subtracting 7 dollars from the remaining money until there is not enough money left or no more children left to distribute the money to.</li><li>After the loop, we check the edge cases. <ul><li>If there is any remaining money left after all the children have received their share, it subtracts one from our result.</li><li>If there is only one child left to receive money, and there is exactly three units of money left to distribute. In this case, we also need to subtract one from our result.</li></ul></li></ul></li></ul><h4 id="complexity" tabindex="-1"><a class="header-anchor" href="#complexity" aria-hidden="true">#</a> Complexity</h4><ul><li>Time complexity: O(money / 7)</li><li>Space complexity: O(1)</li></ul><h4 id="code" tabindex="-1"><a class="header-anchor" href="#code" aria-hidden="true">#</a> Code</h4><div class="language-cpp ext-cpp line-numbers-mode"><pre class="language-cpp"><code><span class="token keyword">int</span> <span class="token function">distMoney</span><span class="token punctuation">(</span><span class="token keyword">int</span> money<span class="token punctuation">,</span> <span class="token keyword">int</span> children<span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">if</span> <span class="token punctuation">(</span>money <span class="token operator">&lt;</span> children<span class="token punctuation">)</span> <span class="token keyword">return</span> <span class="token operator">-</span><span class="token number">1</span><span class="token punctuation">;</span>

    <span class="token keyword">int</span> res <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span>
    money <span class="token operator">-=</span> children<span class="token punctuation">;</span>
    <span class="token keyword">while</span> <span class="token punctuation">(</span>money <span class="token operator">&gt;=</span> <span class="token number">7</span> <span class="token operator">&amp;&amp;</span> children<span class="token punctuation">)</span>
    <span class="token punctuation">{</span>
        money <span class="token operator">-=</span> <span class="token number">7</span><span class="token punctuation">;</span>
        children<span class="token operator">--</span><span class="token punctuation">;</span>
        res<span class="token operator">++</span><span class="token punctuation">;</span>
    <span class="token punctuation">}</span>

    <span class="token keyword">if</span> <span class="token punctuation">(</span>res<span class="token punctuation">)</span>
    <span class="token punctuation">{</span>
        <span class="token keyword">if</span> <span class="token punctuation">(</span>children <span class="token operator">==</span> <span class="token number">0</span> <span class="token operator">&amp;&amp;</span> money<span class="token punctuation">)</span> <span class="token operator">--</span>res<span class="token punctuation">;</span>
        <span class="token keyword">if</span> <span class="token punctuation">(</span>children <span class="token operator">==</span> <span class="token number">1</span> <span class="token operator">&amp;&amp;</span> money <span class="token operator">==</span> <span class="token number">3</span><span class="token punctuation">)</span> <span class="token operator">--</span>res<span class="token punctuation">;</span>
    <span class="token punctuation">}</span>
    <span class="token keyword">return</span> res<span class="token punctuation">;</span>
<span class="token punctuation">}</span>
</code></pre><div class="line-numbers" aria-hidden="true"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><h2 id="q2-maximize-greatness-of-an-array" tabindex="-1"><a class="header-anchor" href="#q2-maximize-greatness-of-an-array" aria-hidden="true">#</a> Q2 Maximize Greatness of an Array</h2><h3 id="solution-1-1" tabindex="-1"><a class="header-anchor" href="#solution-1-1" aria-hidden="true">#</a> Solution 1</h3><h4 id="intuition-1" tabindex="-1"><a class="header-anchor" href="#intuition-1" aria-hidden="true">#</a> Intuition</h4><h4 id="approach-1" tabindex="-1"><a class="header-anchor" href="#approach-1" aria-hidden="true">#</a> Approach</h4><h4 id="complexity-1" tabindex="-1"><a class="header-anchor" href="#complexity-1" aria-hidden="true">#</a> Complexity</h4><ul><li>Time complexity: O(n)</li><li>Space complexity: O(1)</li></ul><h4 id="code-1" tabindex="-1"><a class="header-anchor" href="#code-1" aria-hidden="true">#</a> Code</h4><div class="language-cpp ext-cpp line-numbers-mode"><pre class="language-cpp"><code>
</code></pre><div class="line-numbers" aria-hidden="true"><div class="line-number"></div></div></div><h3 id="solution-2" tabindex="-1"><a class="header-anchor" href="#solution-2" aria-hidden="true">#</a> Solution 2</h3><h4 id="intuition-2" tabindex="-1"><a class="header-anchor" href="#intuition-2" aria-hidden="true">#</a> Intuition</h4><h4 id="approach-2" tabindex="-1"><a class="header-anchor" href="#approach-2" aria-hidden="true">#</a> Approach</h4><h4 id="complexity-2" tabindex="-1"><a class="header-anchor" href="#complexity-2" aria-hidden="true">#</a> Complexity</h4><ul><li>Time complexity: O(1)</li><li>Space complexity: O(1)</li></ul><h4 id="code-2" tabindex="-1"><a class="header-anchor" href="#code-2" aria-hidden="true">#</a> Code</h4><div class="language-cpp ext-cpp line-numbers-mode"><pre class="language-cpp"><code>
</code></pre><div class="line-numbers" aria-hidden="true"><div class="line-number"></div></div></div><h2 id="q3-find-score-of-an-array-after-marking-all-elements" tabindex="-1"><a class="header-anchor" href="#q3-find-score-of-an-array-after-marking-all-elements" aria-hidden="true">#</a> Q3 Find Score of an Array After Marking All Elements</h2><h3 id="solution-1-2" tabindex="-1"><a class="header-anchor" href="#solution-1-2" aria-hidden="true">#</a> Solution 1</h3><h4 id="intuition-3" tabindex="-1"><a class="header-anchor" href="#intuition-3" aria-hidden="true">#</a> Intuition</h4><h4 id="approach-3" tabindex="-1"><a class="header-anchor" href="#approach-3" aria-hidden="true">#</a> Approach</h4><h4 id="complexity-3" tabindex="-1"><a class="header-anchor" href="#complexity-3" aria-hidden="true">#</a> Complexity</h4><ul><li>Time complexity: O(n)</li><li>Space complexity: O(n)</li></ul><h4 id="code-3" tabindex="-1"><a class="header-anchor" href="#code-3" aria-hidden="true">#</a> Code</h4><div class="language-cpp ext-cpp line-numbers-mode"><pre class="language-cpp"><code>
</code></pre><div class="line-numbers" aria-hidden="true"><div class="line-number"></div></div></div><h2 id="q4-minimum-time-to-repair-cars" tabindex="-1"><a class="header-anchor" href="#q4-minimum-time-to-repair-cars" aria-hidden="true">#</a> Q4 Minimum Time to Repair Cars</h2>`,34),r=[t];function o(l,c){return n(),e("div",null,r)}var p=a(i,[["render",o],["__file","index.html.vue"]]);export{p as default};
