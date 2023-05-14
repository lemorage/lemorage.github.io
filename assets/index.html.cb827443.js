import{_ as n,o as s,c as a,d as e}from"./app.5870d99f.js";const t={},p=e(`<h2 id="q1-number-of-senior-citizens" tabindex="-1"><a class="header-anchor" href="#q1-number-of-senior-citizens" aria-hidden="true">#</a> Q1 Number of Senior Citizens</h2><h3 id="solution-1" tabindex="-1"><a class="header-anchor" href="#solution-1" aria-hidden="true">#</a> Solution 1</h3><h4 id="intuition" tabindex="-1"><a class="header-anchor" href="#intuition" aria-hidden="true">#</a> Intuition</h4><p>To solve this problem, we can iterate over each string in the <code>details</code> array and extract the age information from the string. If the extracted age is strictly greater than 60, we increment a counter. Finally, we return the value of the counter, which represents the number of passengers who are strictly more than 60 years old.</p><h4 id="approach" tabindex="-1"><a class="header-anchor" href="#approach" aria-hidden="true">#</a> Approach</h4><ul><li>Initialize a variable <code>res</code> to keep track of the count of seniors.</li><li>Iterate over each string in the details array.</li><li>Extract the age substring from the string using <code>substr()</code>. This substring represents the age of the person.</li><li>If the extracted age is strictly greater than &quot;60&quot;, increment the res counter.</li><li>Finally, return the value of <code>res</code>.</li></ul><h4 id="complexity" tabindex="-1"><a class="header-anchor" href="#complexity" aria-hidden="true">#</a> Complexity</h4><ul><li>Time complexity: <span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height:1em;vertical-align:-0.25em;"></span><span class="mord mathnormal" style="margin-right:0.02778em;">O</span><span class="mopen">(</span><span class="mord mathnormal">n</span><span class="mclose">)</span></span></span></span></li><li>Space complexity: <span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height:1em;vertical-align:-0.25em;"></span><span class="mord mathnormal" style="margin-right:0.02778em;">O</span><span class="mopen">(</span><span class="mord">1</span><span class="mclose">)</span></span></span></span></li></ul><h4 id="code" tabindex="-1"><a class="header-anchor" href="#code" aria-hidden="true">#</a> Code</h4><div class="language-cpp ext-cpp line-numbers-mode"><pre class="language-cpp"><code><span class="token keyword">int</span> <span class="token function">countSeniors</span><span class="token punctuation">(</span>vector<span class="token operator">&lt;</span>string<span class="token operator">&gt;</span><span class="token operator">&amp;</span> details<span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">int</span> res <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span>
    <span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token keyword">auto</span> d <span class="token operator">:</span> details<span class="token punctuation">)</span>
        <span class="token keyword">if</span> <span class="token punctuation">(</span>d<span class="token punctuation">.</span><span class="token function">substr</span><span class="token punctuation">(</span><span class="token number">11</span><span class="token punctuation">,</span> <span class="token number">2</span><span class="token punctuation">)</span> <span class="token operator">&gt;</span> <span class="token string">&quot;60&quot;</span><span class="token punctuation">)</span>
            <span class="token operator">++</span>res<span class="token punctuation">;</span>
    <span class="token keyword">return</span> res<span class="token punctuation">;</span>
<span class="token punctuation">}</span>
</code></pre><div class="line-numbers" aria-hidden="true"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><h2 id="q2-sum-in-a-matrix" tabindex="-1"><a class="header-anchor" href="#q2-sum-in-a-matrix" aria-hidden="true">#</a> Q2 Sum in a Matrix</h2><h3 id="solution-1-1" tabindex="-1"><a class="header-anchor" href="#solution-1-1" aria-hidden="true">#</a> Solution 1</h3><h4 id="intuition-1" tabindex="-1"><a class="header-anchor" href="#intuition-1" aria-hidden="true">#</a> Intuition</h4><p>The problem requires finding the maximum number in each row of the given matrix, removing it, and adding it to the score. However, in terms of efficiency, we don&#39;t actually need to physically remove the elements. We can achieve the same result by sorting the elements in each row in descending order.</p><h4 id="approach-1" tabindex="-1"><a class="header-anchor" href="#approach-1" aria-hidden="true">#</a> Approach</h4><ul><li>Initialize the score <code>res</code> as 0.</li><li>Iterate over each column <code>j</code> in the matrix.</li><li>For each column, find the maximum number <code>x</code> in each row. <ul><li>If it&#39;s the first column <code>(j = 0)</code>, sort the row in descending order to ensure we select the maximum number.</li><li>Otherwise, we can assume that the row is already sorted in descending order, so we don&#39;t need to sort it again.</li></ul></li><li>Add <code>x</code> to the score <code>res</code>.</li><li>Repeat and finally return the final score <code>res</code>.</li></ul><h4 id="complexity-1" tabindex="-1"><a class="header-anchor" href="#complexity-1" aria-hidden="true">#</a> Complexity</h4><p>n is the number of rows and m is the number of columns</p><ul><li>Time complexity: <span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height:1em;vertical-align:-0.25em;"></span><span class="mord mathnormal" style="margin-right:0.02778em;">O</span><span class="mopen">(</span><span class="mord mathnormal">n</span><span class="mspace" style="margin-right:0.2222em;"></span><span class="mbin">\u2217</span><span class="mspace" style="margin-right:0.2222em;"></span></span><span class="base"><span class="strut" style="height:1em;vertical-align:-0.25em;"></span><span class="mord mathnormal">m</span><span class="mspace" style="margin-right:0.1667em;"></span><span class="mop">lo<span style="margin-right:0.01389em;">g</span></span><span class="mopen">(</span><span class="mord mathnormal">m</span><span class="mclose">))</span></span></span></span></li><li>Space complexity: <span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height:1em;vertical-align:-0.25em;"></span><span class="mord mathnormal" style="margin-right:0.02778em;">O</span><span class="mopen">(</span><span class="mord">1</span><span class="mclose">)</span></span></span></span></li></ul><h4 id="code-1" tabindex="-1"><a class="header-anchor" href="#code-1" aria-hidden="true">#</a> Code</h4><div class="language-cpp ext-cpp line-numbers-mode"><pre class="language-cpp"><code><span class="token keyword">int</span> <span class="token function">matrixSum</span><span class="token punctuation">(</span>vector<span class="token operator">&lt;</span>vector<span class="token operator">&lt;</span><span class="token keyword">int</span><span class="token operator">&gt;&gt;</span><span class="token operator">&amp;</span> nums<span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">int</span> res <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span>
    
    <span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token keyword">int</span> j <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span> j <span class="token operator">&lt;</span> nums<span class="token punctuation">[</span><span class="token number">0</span><span class="token punctuation">]</span><span class="token punctuation">.</span><span class="token function">size</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span> <span class="token operator">++</span>j<span class="token punctuation">)</span>
    <span class="token punctuation">{</span>
        <span class="token keyword">int</span> x <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span>
        <span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token keyword">int</span> i <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span> i <span class="token operator">&lt;</span> nums<span class="token punctuation">.</span><span class="token function">size</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span> <span class="token operator">++</span>i<span class="token punctuation">)</span>
        <span class="token punctuation">{</span>
            <span class="token keyword">if</span> <span class="token punctuation">(</span><span class="token operator">!</span>j<span class="token punctuation">)</span> <span class="token function">sort</span><span class="token punctuation">(</span>nums<span class="token punctuation">[</span>i<span class="token punctuation">]</span><span class="token punctuation">.</span><span class="token function">rbegin</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">,</span> nums<span class="token punctuation">[</span>i<span class="token punctuation">]</span><span class="token punctuation">.</span><span class="token function">rend</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
            x <span class="token operator">=</span> <span class="token function">max</span><span class="token punctuation">(</span>x<span class="token punctuation">,</span> nums<span class="token punctuation">[</span>i<span class="token punctuation">]</span><span class="token punctuation">[</span>j<span class="token punctuation">]</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
        <span class="token punctuation">}</span>
        res <span class="token operator">+=</span> x<span class="token punctuation">;</span>
    <span class="token punctuation">}</span>
    <span class="token keyword">return</span> res<span class="token punctuation">;</span>
<span class="token punctuation">}</span>
</code></pre><div class="line-numbers" aria-hidden="true"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><h2 id="q3-maximum-or" tabindex="-1"><a class="header-anchor" href="#q3-maximum-or" aria-hidden="true">#</a> Q3 Maximum OR</h2><h3 id="solution-1-2" tabindex="-1"><a class="header-anchor" href="#solution-1-2" aria-hidden="true">#</a> Solution 1</h3><h4 id="intuition-2" tabindex="-1"><a class="header-anchor" href="#intuition-2" aria-hidden="true">#</a> Intuition</h4><p>The goal is to maximize the value obtained after applying the operation of doubling an element in the array. To achieve this, we want to left shift only the same number <code>k</code> times.</p><h4 id="approach-2" tabindex="-1"><a class="header-anchor" href="#approach-2" aria-hidden="true">#</a> Approach</h4><p>To determine the maximum value, we need to consider the effect of doubling an element. Let&#39;s define two arrays:</p><ul><li><code>suffix[i]</code> represents the product of the elements <code>nums[i+1] * nums[i+2] * ... * nums[n-1]</code>.</li><li><code>prefix[i]</code> represents the product of the elements <code>nums[0] * nums[1] * ... * nums[i-1]</code>.</li><li>The result of doubling <code>nums[i]</code> can be calculated as <strong><code>prefix[i] | (nums[i] &lt;&lt; k) | suffix[i]</code></strong>. This formula takes into account the left side, the doubled value, and the right side of <code>nums[i]</code>.</li><li>We can iterate through the array <code>nums</code> and calculate the maximum result by comparing it with the previous maximum. To keep track of the left side, we use a variable <code>pre</code> that accumulates the bitwise OR of the elements encountered so far.</li></ul><h4 id="complexity-2" tabindex="-1"><a class="header-anchor" href="#complexity-2" aria-hidden="true">#</a> Complexity</h4><ul><li>Time complexity: <span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height:1em;vertical-align:-0.25em;"></span><span class="mord mathnormal" style="margin-right:0.02778em;">O</span><span class="mopen">(</span><span class="mord mathnormal">n</span><span class="mclose">)</span></span></span></span>, where n is the length of the input array <code>nums</code>.</li><li>Space complexity: <span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height:1em;vertical-align:-0.25em;"></span><span class="mord mathnormal" style="margin-right:0.02778em;">O</span><span class="mopen">(</span><span class="mord mathnormal">n</span><span class="mclose">)</span></span></span></span> for the auxiliary array <code>suf</code></li></ul><h4 id="code-2" tabindex="-1"><a class="header-anchor" href="#code-2" aria-hidden="true">#</a> Code</h4><div class="language-cpp ext-cpp line-numbers-mode"><pre class="language-cpp"><code><span class="token keyword">long</span> <span class="token keyword">long</span> <span class="token function">maximumOr</span><span class="token punctuation">(</span>vector<span class="token operator">&lt;</span><span class="token keyword">int</span><span class="token operator">&gt;</span><span class="token operator">&amp;</span> nums<span class="token punctuation">,</span> <span class="token keyword">int</span> k<span class="token punctuation">)</span> <span class="token punctuation">{</span>
    <span class="token keyword">int</span> n <span class="token operator">=</span> nums<span class="token punctuation">.</span><span class="token function">size</span><span class="token punctuation">(</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    vector<span class="token operator">&lt;</span><span class="token keyword">int</span><span class="token operator">&gt;</span> <span class="token function">suf</span><span class="token punctuation">(</span>n <span class="token operator">+</span> <span class="token number">1</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
    <span class="token keyword">long</span> <span class="token keyword">long</span> res <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span>

    <span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token keyword">int</span> i <span class="token operator">=</span> n <span class="token operator">-</span> <span class="token number">1</span><span class="token punctuation">;</span> i<span class="token punctuation">;</span> <span class="token operator">--</span>i<span class="token punctuation">)</span>
        suf<span class="token punctuation">[</span>i<span class="token punctuation">]</span> <span class="token operator">=</span> suf<span class="token punctuation">[</span>i <span class="token operator">+</span> <span class="token number">1</span><span class="token punctuation">]</span> <span class="token operator">|</span> nums<span class="token punctuation">[</span>i<span class="token punctuation">]</span><span class="token punctuation">;</span>

    <span class="token keyword">for</span> <span class="token punctuation">(</span><span class="token keyword">int</span> i <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">,</span> pre <span class="token operator">=</span> <span class="token number">0</span><span class="token punctuation">;</span> i <span class="token operator">&lt;</span> n<span class="token punctuation">;</span> <span class="token operator">++</span>i<span class="token punctuation">)</span>
    <span class="token punctuation">{</span>
        res <span class="token operator">=</span> <span class="token function">max</span><span class="token punctuation">(</span>res<span class="token punctuation">,</span> pre <span class="token operator">|</span> <span class="token number">1LL</span> <span class="token operator">*</span> nums<span class="token punctuation">[</span>i<span class="token punctuation">]</span> <span class="token operator">&lt;&lt;</span> k <span class="token operator">|</span> suf<span class="token punctuation">[</span>i <span class="token operator">+</span> <span class="token number">1</span><span class="token punctuation">]</span><span class="token punctuation">)</span><span class="token punctuation">;</span>
        pre <span class="token operator">|=</span> nums<span class="token punctuation">[</span>i<span class="token punctuation">]</span><span class="token punctuation">;</span>
    <span class="token punctuation">}</span>
    <span class="token keyword">return</span> res<span class="token punctuation">;</span>
<span class="token punctuation">}</span>
</code></pre><div class="line-numbers" aria-hidden="true"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><h2 id="q4-power-of-heroes-tbd" tabindex="-1"><a class="header-anchor" href="#q4-power-of-heroes-tbd" aria-hidden="true">#</a> Q4 Power of Heroes(TBD...)</h2>`,33),o=[p];function i(c,l){return s(),a("div",null,o)}var u=n(t,[["render",i],["__file","index.html.vue"]]);export{u as default};