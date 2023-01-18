pub const NotLinkedConst:        usize = 0;
pub const QtAutoConnection:        u32 = NotLinkedConst as u32 ;//TODO
pub const QtDisplayRole:         usize = NotLinkedConst;//TODO
pub const QtQueuedConnection:      u32 = NotLinkedConst as u32;//TODO
pub const QtUseRole:             usize = NotLinkedConst;//TODO
pub const QtWindowCloseButtonHint: u32 = NotLinkedConst as u32 ;//TODO
pub const QtWindowSystemMenuHint:  u32 = NotLinkedConst as u32 ;//TODO
pub const QtWindowTitleHint:       u32 = NotLinkedConst as u32 ;//TODO
pub const QtAscendingOrder:        u32 = NotLinkedConst as u32 ;//TODO
pub const QtWidgetDefault:         u32 = NotLinkedConst as u32 ;//TODO

pub struct QList<T> {
    p: std::marker::PhantomData<T> 
}

pub struct QQueue<T> {
    p: std::marker::PhantomData<T> 
}

pub struct QMap<T,S> {
    p: std::marker::PhantomData<T>,
    s: std::marker::PhantomData<S>,
}

pub type AddressTableEntryEditStatus     = NotLinked;
pub type Exception                       = NotLinked;
pub type OptionsDialogTab                = NotLinked;
pub type NotLinked                       = u32;
pub type QAbstractButton                 = NotLinked;
pub type QAbstractItemDelegate           = NotLinked;
pub type QtAscendingOrder                = NotLinked;
pub type QAbstractItemModel              = NotLinked;
pub type QAbstractItemView               = NotLinked;
pub type QAbstractListModel              = NotLinked;
pub type QAbstractSpinBox                = NotLinked;
pub type QAbstractTableModel             = NotLinked;
pub type QAction                         = NotLinked;
pub type QApplication                    = NotLinked;
pub type QByteArray                      = NotLinked;
pub type QCloseEvent                     = NotLinked;
pub type QComboBox                       = NotLinked;
pub type QCompleter                      = NotLinked;
pub type QContextMenuEvent               = NotLinked;
pub type QDataWidgetMapper               = NotLinked;
pub type QDate                           = NotLinked;
pub type QDateTime                       = NotLinked;
pub type QDateTimeEdit                   = NotLinked;
pub type QDialog                         = NotLinked;
pub type QDragEnterEvent                 = NotLinked;
pub type QDropEvent                      = NotLinked;
pub type QEvent                          = NotLinked;
pub type QFocusEvent                     = NotLinked;
pub type QFont                           = NotLinked;
pub type QFontMetrics                    = NotLinked;
pub type QFrame                          = NotLinked;
pub type QHideEvent                      = NotLinked;
pub type QImage                          = NotLinked;
pub type QItemDelegate                   = NotLinked;
pub type QItemFlags                      = NotLinked;
pub type QItemSelection                  = NotLinked;
pub type QKeyEvent                       = NotLinked;
pub type QKeySequence                    = NotLinked;
pub type QLabel                          = NotLinked;
pub type QLineEdit                       = NotLinked;
pub type QListView                       = NotLinked;
pub type QLocalServer                    = NotLinked;
pub type QLocale                         = NotLinked;
pub type QMainWindow                     = NotLinked;
pub type QMenu                           = NotLinked;
pub type QMenuBar                        = NotLinked;
pub type QMessageBox                     = NotLinked;
pub type QMessageLogContext              = NotLinked;
pub type QMetatype                       = NotLinked;
pub type QModel                          = NotLinked;
pub type QMouseEvent                     = NotLinked;
pub type QMsgType                        = NotLinked;
pub type QMutex                          = NotLinked;
pub type QObject                         = NotLinked;
pub type QOrientation                    = NotLinked;
pub type QPaintEvent                     = NotLinked;
pub type QPainter                        = NotLinked;
pub type QPainterPath                    = NotLinked;
pub type QPixmap                         = NotLinked;
pub type QPoint                          = NotLinked;
pub type QProgressBar                    = NotLinked;
pub type QProgressDialog                 = NotLinked;
pub type QPropertyAnimation              = NotLinked;
pub type QResizeEvent                    = NotLinked;
pub type QSettings                       = NotLinked;
pub type QShowEvent                      = NotLinked;
pub type QSortFilterProxyModel           = NotLinked;
pub type QSortOrder                      = NotLinked;
pub type QStackedWidget                  = NotLinked;
pub type QStringList                     = NotLinked;
pub type QStyleOptionViewItem            = NotLinked;
pub type QStyledItemDelegate             = NotLinked;
pub type QSystemTrayIcon                 = NotLinked;
pub type QSystemTrayIconActivationReason = NotLinked;
pub type QTableView                      = NotLinked;
pub type QTextStream                     = NotLinked;
pub type QThread                         = NotLinked;
pub type QTimer                          = NotLinked;
pub type QToolBar                        = NotLinked;
pub type QTranslator                     = NotLinked;
pub type QTreeWidget                     = NotLinked;
pub type QTreeWidgetItem                 = NotLinked;
pub type QUrl                            = NotLinked;
pub type QUserRole                       = NotLinked;
pub type QValidator                      = NotLinked;
pub type QValidatorState                 = NotLinked;
pub type QVariant                        = NotLinked;
pub type QVector                         = NotLinked;
pub type QWidget                         = NotLinked;
pub type QtConnectionType                = NotLinked;
pub type QtItemFlags                     = NotLinked;
pub type QtMsgType                       = NotLinked;
pub type QtOrientation                   = NotLinked;
pub type QtSortOrder                     = NotLinked;
pub type QtWindowFlags                   = NotLinked;
pub type StepEnabled                     = NotLinked;
pub type UiAddressBookPage               = NotLinked;
pub type UiAskPassphraseDialog           = NotLinked;
pub type UiCoinControlDialog             = NotLinked;
pub type UiCreateWalletDialog            = NotLinked;
pub type UiEditAddressDialog             = NotLinked;
pub type UiHelpMessageDialog             = NotLinked;
pub type UiIntro                         = NotLinked;
pub type UiModalOverlay                  = NotLinked;
pub type UiOpenURIDialog                 = NotLinked;
pub type UiOptionsDialog                 = NotLinked;
pub type UiOverviewPage                  = NotLinked;
pub type UiPSBTOperationsDialog          = NotLinked;
pub type UiRPCConsole                    = NotLinked;
pub type UiReceiveCoinsDialog            = NotLinked;
pub type UiReceiveRequestDialog          = NotLinked;
pub type UiSendCoinsDialog               = NotLinked;
pub type UiSendCoinsEntry                = NotLinked;
pub type UiSignVerifyMessageDialog       = NotLinked;
pub type UiTransactionDescDialog         = NotLinked;
pub type WId                             = NotLinked;

pub(crate) use bitcoin_amt::*;
pub(crate) use bitcoin_argsman::*;
pub(crate) use bitcoin_derive::*;
pub(crate) use bitcoin_imports::*;
pub(crate) use bitcoin_index::*;
pub(crate) use bitcoin_net::*;
pub(crate) use bitcoin_node::*;
pub(crate) use bitcoin_peerman::*;
pub(crate) use bitcoin_primitives::*;
pub(crate) use bitcoin_psbt::*;
pub(crate) use bitcoin_scripting::*;
pub(crate) use bitcoin_signingprovider::*;
pub(crate) use bitcoin_string::*;
pub(crate) use bitcoin_support::*;
pub(crate) use bitcoin_txmempool::*;
pub(crate) use bitcoin_indexed_chain::*;
pub(crate) use bitcoin_sam::*;
pub(crate) use bitcoin_tx::*;
pub(crate) use bitcoinrpc_util::*;
pub(crate) use bitcoin_client_ui::*;
pub(crate) use bitcoinrpc_server::*;
pub(crate) use bitcoin_subnet::*;
pub(crate) use bitcoin_banman::*;
pub(crate) use bitcoinchain_params::*;
pub(crate) use bitcoin_network::*;
pub(crate) use bitcoinnode_stats::*;
pub(crate) use bitcoinnode_interface::*;
pub(crate) use bitcoin_block::*;
pub(crate) use bitcoinwallet_interface::*;
pub(crate) use bitcoinwallet_library::*;

