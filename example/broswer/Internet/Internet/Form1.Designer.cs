namespace Internet
{
    partial class Form1
    {
        /// <summary>
        /// 必需的设计器变量。
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// 清理所有正在使用的资源。
        /// </summary>
        /// <param name="disposing">如果应释放托管资源，为 true；否则为 false。</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows 窗体设计器生成的代码

        /// <summary>
        /// 设计器支持所需的方法 - 不要
        /// 使用代码编辑器修改此方法的内容。
        /// </summary>
        private void InitializeComponent()
        {
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(Form1));
            this.tableLayoutPanel1 = new System.Windows.Forms.TableLayoutPanel();
            this.toolStrip1 = new System.Windows.Forms.ToolStrip();
            this.back = new System.Windows.Forms.ToolStripButton();
            this.foward = new System.Windows.Forms.ToolStripButton();
            this.stop = new System.Windows.Forms.ToolStripButton();
            this.refresh = new System.Windows.Forms.ToolStripButton();
            this.home = new System.Windows.Forms.ToolStripButton();
            this.print = new System.Windows.Forms.ToolStripButton();
            this.save = new System.Windows.Forms.ToolStripButton();
            this.label2 = new System.Windows.Forms.Label();
            this.go = new System.Windows.Forms.Button();
            this.urlBox = new System.Windows.Forms.TextBox();
            this.webBrowser1 = new System.Windows.Forms.WebBrowser();
            this.tableLayoutPanel1.SuspendLayout();
            this.toolStrip1.SuspendLayout();
            this.SuspendLayout();
            // 
            // tableLayoutPanel1
            // 
            this.tableLayoutPanel1.ColumnCount = 3;
            this.tableLayoutPanel1.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 6.57277F));
            this.tableLayoutPanel1.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 93.42723F));
            this.tableLayoutPanel1.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Absolute, 83F));
            this.tableLayoutPanel1.Controls.Add(this.toolStrip1, 0, 0);
            this.tableLayoutPanel1.Controls.Add(this.label2, 0, 1);
            this.tableLayoutPanel1.Controls.Add(this.go, 2, 1);
            this.tableLayoutPanel1.Controls.Add(this.urlBox, 1, 1);
            this.tableLayoutPanel1.Dock = System.Windows.Forms.DockStyle.Top;
            this.tableLayoutPanel1.Location = new System.Drawing.Point(0, 0);
            this.tableLayoutPanel1.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
            this.tableLayoutPanel1.Name = "tableLayoutPanel1";
            this.tableLayoutPanel1.RowCount = 2;
            this.tableLayoutPanel1.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 66.15385F));
            this.tableLayoutPanel1.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 33.84615F));
            this.tableLayoutPanel1.Size = new System.Drawing.Size(1175, 75);
            this.tableLayoutPanel1.TabIndex = 1;
            this.tableLayoutPanel1.Paint += new System.Windows.Forms.PaintEventHandler(this.tableLayoutPanel1_Paint);
            // 
            // toolStrip1
            // 
            this.toolStrip1.BackColor = System.Drawing.SystemColors.ButtonHighlight;
            this.tableLayoutPanel1.SetColumnSpan(this.toolStrip1, 3);
            this.toolStrip1.Font = new System.Drawing.Font("Tahoma", 11.25F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.toolStrip1.ImageScalingSize = new System.Drawing.Size(20, 20);
            this.toolStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[] {
            this.back,
            this.foward,
            this.stop,
            this.refresh,
            this.home,
            this.print,
            this.save});
            this.toolStrip1.Location = new System.Drawing.Point(0, 0);
            this.toolStrip1.Name = "toolStrip1";
            this.toolStrip1.RenderMode = System.Windows.Forms.ToolStripRenderMode.Professional;
            this.toolStrip1.Size = new System.Drawing.Size(1175, 44);
            this.toolStrip1.TabIndex = 0;
            this.toolStrip1.Text = "toolStrip1";
            this.toolStrip1.ItemClicked += new System.Windows.Forms.ToolStripItemClickedEventHandler(this.toolStrip1_ItemClicked);
            // 
            // back
            // 
            this.back.Image = ((System.Drawing.Image)(resources.GetObject("back.Image")));
            this.back.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.back.Margin = new System.Windows.Forms.Padding(0, 0, 8, 2);
            this.back.Name = "back";
            this.back.Size = new System.Drawing.Size(42, 42);
            this.back.Text = "后退";
            this.back.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageAboveText;
            this.back.ToolTipText = "后退";
            this.back.Click += new System.EventHandler(this.back_Click);
            // 
            // foward
            // 
            this.foward.Image = ((System.Drawing.Image)(resources.GetObject("foward.Image")));
            this.foward.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.foward.Margin = new System.Windows.Forms.Padding(0, 0, 8, 2);
            this.foward.Name = "foward";
            this.foward.Size = new System.Drawing.Size(42, 42);
            this.foward.Text = "前进";
            this.foward.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageAboveText;
            this.foward.Click += new System.EventHandler(this.foward_Click);
            // 
            // stop
            // 
            this.stop.Image = ((System.Drawing.Image)(resources.GetObject("stop.Image")));
            this.stop.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.stop.Margin = new System.Windows.Forms.Padding(0, 0, 8, 2);
            this.stop.Name = "stop";
            this.stop.Size = new System.Drawing.Size(42, 42);
            this.stop.Text = "停止";
            this.stop.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageAboveText;
            this.stop.Click += new System.EventHandler(this.stop_Click);
            // 
            // refresh
            // 
            this.refresh.Image = ((System.Drawing.Image)(resources.GetObject("refresh.Image")));
            this.refresh.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.refresh.Margin = new System.Windows.Forms.Padding(0, 0, 8, 2);
            this.refresh.Name = "refresh";
            this.refresh.Size = new System.Drawing.Size(42, 42);
            this.refresh.Text = "刷新";
            this.refresh.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageAboveText;
            this.refresh.Click += new System.EventHandler(this.refresh_Click);
            // 
            // home
            // 
            this.home.Image = ((System.Drawing.Image)(resources.GetObject("home.Image")));
            this.home.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.home.Margin = new System.Windows.Forms.Padding(0, 0, 8, 2);
            this.home.Name = "home";
            this.home.Size = new System.Drawing.Size(42, 42);
            this.home.Text = "主页";
            this.home.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageAboveText;
            this.home.Click += new System.EventHandler(this.toolStripButton5_Click);
            // 
            // print
            // 
            this.print.Image = ((System.Drawing.Image)(resources.GetObject("print.Image")));
            this.print.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.print.Margin = new System.Windows.Forms.Padding(0, 0, 8, 2);
            this.print.Name = "print";
            this.print.Size = new System.Drawing.Size(42, 42);
            this.print.Text = "打印";
            this.print.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageAboveText;
            this.print.Click += new System.EventHandler(this.print_Click);
            // 
            // save
            // 
            this.save.Image = ((System.Drawing.Image)(resources.GetObject("save.Image")));
            this.save.ImageTransparentColor = System.Drawing.Color.Magenta;
            this.save.Margin = new System.Windows.Forms.Padding(0, 0, 0, 2);
            this.save.Name = "save";
            this.save.Size = new System.Drawing.Size(57, 42);
            this.save.Text = "另存为";
            this.save.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageAboveText;
            this.save.Click += new System.EventHandler(this.save_Click);
            // 
            // label2
            // 
            this.label2.AutoSize = true;
            this.label2.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(192)))), ((int)(((byte)(255)))), ((int)(((byte)(255)))));
            this.label2.Dock = System.Windows.Forms.DockStyle.Fill;
            this.label2.FlatStyle = System.Windows.Forms.FlatStyle.System;
            this.label2.ForeColor = System.Drawing.SystemColors.ControlText;
            this.label2.Location = new System.Drawing.Point(3, 49);
            this.label2.Name = "label2";
            this.label2.Size = new System.Drawing.Size(65, 26);
            this.label2.TabIndex = 1;
            this.label2.Text = "地址";
            this.label2.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            this.label2.Click += new System.EventHandler(this.label1_Click);
            // 
            // go
            // 
            this.go.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right)));
            this.go.Image = ((System.Drawing.Image)(resources.GetObject("go.Image")));
            this.go.Location = new System.Drawing.Point(1092, 50);
            this.go.Margin = new System.Windows.Forms.Padding(1);
            this.go.Name = "go";
            this.go.Size = new System.Drawing.Size(82, 24);
            this.go.TabIndex = 3;
            this.go.Text = "转到";
            this.go.TextImageRelation = System.Windows.Forms.TextImageRelation.ImageBeforeText;
            this.go.UseVisualStyleBackColor = true;
            this.go.Click += new System.EventHandler(this.go_Click);
            // 
            // urlBox
            // 
            this.urlBox.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Left | System.Windows.Forms.AnchorStyles.Right)));
            this.urlBox.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(192)))), ((int)(((byte)(255)))), ((int)(((byte)(255)))));
            this.urlBox.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
            this.urlBox.Location = new System.Drawing.Point(72, 50);
            this.urlBox.Margin = new System.Windows.Forms.Padding(1);
            this.urlBox.Multiline = true;
            this.urlBox.Name = "urlBox";
            this.urlBox.Size = new System.Drawing.Size(1018, 24);
            this.urlBox.TabIndex = 2;
            this.urlBox.TextChanged += new System.EventHandler(this.urlBox_TextChanged);
            this.urlBox.KeyPress += new System.Windows.Forms.KeyPressEventHandler(this.urlBox_KeyPress);
            // 
            // webBrowser1
            // 
            this.webBrowser1.Dock = System.Windows.Forms.DockStyle.Fill;
            this.webBrowser1.Location = new System.Drawing.Point(0, 75);
            this.webBrowser1.MinimumSize = new System.Drawing.Size(20, 20);
            this.webBrowser1.Name = "webBrowser1";
            this.webBrowser1.Size = new System.Drawing.Size(1175, 538);
            this.webBrowser1.TabIndex = 2;
            this.webBrowser1.DocumentCompleted += new System.Windows.Forms.WebBrowserDocumentCompletedEventHandler(this.webBrowser1_DocumentCompleted_1);
            // 
            // Form1
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(7F, 14F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.BackColor = System.Drawing.SystemColors.ActiveBorder;
            this.ClientSize = new System.Drawing.Size(1175, 613);
            this.Controls.Add(this.webBrowser1);
            this.Controls.Add(this.tableLayoutPanel1);
            this.Font = new System.Drawing.Font("宋体", 10.5F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(134)));
            this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
            this.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
            this.Name = "Form1";
            this.Text = "WebBrowser";
            this.WindowState = System.Windows.Forms.FormWindowState.Maximized;
            this.Load += new System.EventHandler(this.Form1_Load);
            this.tableLayoutPanel1.ResumeLayout(false);
            this.tableLayoutPanel1.PerformLayout();
            this.toolStrip1.ResumeLayout(false);
            this.toolStrip1.PerformLayout();
            this.ResumeLayout(false);

        }

        #endregion

        private System.Windows.Forms.TableLayoutPanel tableLayoutPanel1;
        private System.Windows.Forms.ToolStrip toolStrip1;
        private System.Windows.Forms.ToolStripButton back;
        private System.Windows.Forms.ToolStripButton foward;
        private System.Windows.Forms.ToolStripButton stop;
        private System.Windows.Forms.ToolStripButton refresh;
        private System.Windows.Forms.ToolStripButton home;
        private System.Windows.Forms.ToolStripButton print;
        private System.Windows.Forms.ToolStripButton save;
        private System.Windows.Forms.Label label2;
        private System.Windows.Forms.Button go;
        public System.Windows.Forms.TextBox urlBox;
        private System.Windows.Forms.WebBrowser webBrowser1;
    }
}

